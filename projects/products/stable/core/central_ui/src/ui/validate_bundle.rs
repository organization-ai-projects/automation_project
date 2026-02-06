//projects/products/core/central_ui/src/ui/validate_bundle.rs
use std::path::Path;

pub(crate) fn validate_bundle(ui_dist: &Path) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let index = ui_dist.join("public").join("index.html");
    if !index.exists() {
        missing.push(index.display().to_string());
    }

    let assets_dir = ui_dist.join("public").join("assets");
    let js_found = std::fs::read_dir(&assets_dir).ok().and_then(|mut entries| {
        entries.find_map(|entry| {
            entry.ok().and_then(|entry| {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("js") {
                    Some(path)
                } else {
                    None
                }
            })
        })
    });
    if js_found.is_none() {
        missing.push(assets_dir.join("*.js").display().to_string());
    }

    let wasm_found = std::fs::read_dir(&assets_dir).ok().and_then(|mut entries| {
        entries.find_map(|entry| {
            entry.ok().and_then(|entry| {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("wasm") {
                    Some(path)
                } else {
                    None
                }
            })
        })
    });
    if wasm_found.is_none() {
        missing.push(assets_dir.join("*.wasm").display().to_string());
    }

    let manifest = ui_dist.join("ui_manifest.ron");
    if !manifest.exists() {
        missing.push(manifest.display().to_string());
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}
