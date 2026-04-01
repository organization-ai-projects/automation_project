use std::collections::BTreeSet;
use std::path::Path;

use crate::crate_graph::CrateGraph;
use crate::crate_node::CrateNode;
use crate::diagnostics::Error;
use crate::module_node::ModuleNode;
use crate::public_item::{ItemKind, PublicItem};
use crate::snapshot::Snapshot;

pub struct WorkspaceScanner;

impl WorkspaceScanner {
    pub fn scan(root_path: &str) -> Result<Snapshot, Error> {
        let root = Path::new(root_path);
        let workspace_toml_path = root.join("Cargo.toml");
        let workspace_content = std::fs::read_to_string(&workspace_toml_path)
            .map_err(|e| Error::Io(format!("{}: {}", workspace_toml_path.display(), e)))?;

        let workspace_value: toml::Value = workspace_content
            .parse()
            .map_err(|e: toml::de::Error| Error::Parse(e.to_string()))?;

        let members = workspace_value
            .get("workspace")
            .and_then(|w| w.get("members"))
            .and_then(|m| m.as_array())
            .ok_or_else(|| Error::Parse("missing [workspace].members".into()))?;

        let member_paths: Vec<String> = members
            .iter()
            .filter_map(|m| m.as_str().map(String::from))
            .collect();

        let all_crate_names: BTreeSet<String> = member_paths
            .iter()
            .filter_map(|mp| read_package_name(root, mp).ok())
            .collect();

        let mut crate_nodes = Vec::new();
        let mut modules = Vec::new();
        let mut public_items = Vec::new();

        for member_path in &member_paths {
            let crate_toml_path = root.join(member_path).join("Cargo.toml");
            let crate_content = std::fs::read_to_string(&crate_toml_path)
                .map_err(|e| Error::Io(format!("{}: {}", crate_toml_path.display(), e)))?;

            let crate_value: toml::Value = crate_content
                .parse()
                .map_err(|e: toml::de::Error| Error::Parse(e.to_string()))?;

            let crate_name = crate_value
                .get("package")
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .ok_or_else(|| Error::Parse(format!("missing [package].name in {member_path}")))?
                .to_string();

            let mut deps: Vec<String> = Vec::new();
            if let Some(dep_table) = crate_value.get("dependencies").and_then(|d| d.as_table()) {
                for dep_name in dep_table.keys() {
                    if all_crate_names.contains(dep_name) {
                        deps.push(dep_name.clone());
                    }
                }
            }
            deps.sort();

            crate_nodes.push(CrateNode {
                name: crate_name.clone(),
                path: member_path.clone(),
                dependencies: deps,
            });

            let src_dir = root.join(member_path).join("src");
            if src_dir.is_dir() {
                let rs_files = collect_rs_files(&src_dir)?;
                for rs_file in rs_files {
                    let rel_to_root = rs_file
                        .strip_prefix(root)
                        .unwrap_or(&rs_file)
                        .to_string_lossy()
                        .replace('\\', "/");

                    let module_path = file_to_module_path(&crate_name, member_path, &rel_to_root);

                    let content = std::fs::read_to_string(&rs_file)
                        .map_err(|e| Error::Io(format!("{}: {}", rs_file.display(), e)))?;

                    let items = parse_public_items(&content);

                    let mut item_names: Vec<String> =
                        items.iter().map(|(name, _)| name.clone()).collect();
                    item_names.sort();

                    modules.push(ModuleNode {
                        crate_name: crate_name.clone(),
                        module_path: module_path.clone(),
                        file_path: rel_to_root,
                        public_items: item_names,
                    });

                    for (name, kind) in items {
                        public_items.push(PublicItem {
                            name,
                            kind,
                            crate_name: crate_name.clone(),
                            module_path: module_path.clone(),
                        });
                    }
                }
            }
        }

        let crate_graph = CrateGraph::new(crate_nodes);
        Ok(Snapshot::new(
            root_path.to_string(),
            crate_graph,
            modules,
            public_items,
        ))
    }
}

fn read_package_name(root: &Path, member_path: &str) -> Result<String, Error> {
    let crate_toml = root.join(member_path).join("Cargo.toml");
    let content = std::fs::read_to_string(&crate_toml).map_err(|e| Error::Io(e.to_string()))?;
    let value: toml::Value = content
        .parse()
        .map_err(|e: toml::de::Error| Error::Parse(e.to_string()))?;
    value
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(String::from)
        .ok_or_else(|| Error::Parse("missing package name".into()))
}

fn collect_rs_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, Error> {
    let mut files = Vec::new();
    collect_rs_files_recursive(dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_rs_files_recursive(
    dir: &Path,
    files: &mut Vec<std::path::PathBuf>,
) -> Result<(), Error> {
    let entries =
        std::fs::read_dir(dir).map_err(|e| Error::Io(format!("{}: {}", dir.display(), e)))?;

    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files_recursive(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn file_to_module_path(crate_name: &str, member_path: &str, rel_to_root: &str) -> String {
    let src_prefix = format!("{}/src/", member_path);
    let within_src = rel_to_root.strip_prefix(&src_prefix).unwrap_or(rel_to_root);

    let without_ext = within_src.strip_suffix(".rs").unwrap_or(within_src);

    if without_ext == "lib" || without_ext == "main" {
        return crate_name.to_string();
    }

    let without_mod = without_ext.strip_suffix("/mod").unwrap_or(without_ext);

    let module_part = without_mod.replace('/', "::");
    format!("{crate_name}::{module_part}")
}

fn parse_public_items(source: &str) -> Vec<(String, ItemKind)> {
    let mut items = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();

        if let Some(name) = try_extract_pub_item(trimmed, "pub fn ", ItemKind::Function) {
            items.push((name, ItemKind::Function));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub struct ", ItemKind::Struct) {
            items.push((name, ItemKind::Struct));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub enum ", ItemKind::Enum) {
            items.push((name, ItemKind::Enum));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub trait ", ItemKind::Trait) {
            items.push((name, ItemKind::Trait));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub type ", ItemKind::TypeAlias) {
            items.push((name, ItemKind::TypeAlias));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub const ", ItemKind::Constant) {
            items.push((name, ItemKind::Constant));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub static ", ItemKind::Static) {
            items.push((name, ItemKind::Static));
        } else if let Some(name) = try_extract_pub_item(trimmed, "pub mod ", ItemKind::Module) {
            items.push((name, ItemKind::Module));
        }
    }

    items.sort();
    items
}

fn try_extract_pub_item(line: &str, prefix: &str, _kind: ItemKind) -> Option<String> {
    if !line.starts_with(prefix) {
        return None;
    }
    let rest = &line[prefix.len()..];
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}
