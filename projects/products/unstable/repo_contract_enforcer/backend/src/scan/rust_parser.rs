use syn::spanned::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryItemViolation {
    pub message: String,
    pub line: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsafeSignals {
    pub has_unsafe_usage: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnderscoreSignals {
    pub has_wildcard_discard: bool,
    pub has_prefixed_binding: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdoutMacroSignals {
    pub has_stdout_macro: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainItemViolationKind {
    Struct,
    Enum,
    Trait,
    Impl,
    NonEntrypointFn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainItemViolation {
    pub kind: MainItemViolationKind,
    pub line: Option<u32>,
}

impl RustParser {
    pub fn first_line_of(haystack: &str, needle: &str) -> Option<u32> {
        haystack
            .find(needle)
            .map(|idx| haystack[..idx].chars().filter(|c| *c == '\n').count() as u32 + 1)
    }

    pub fn first_line_of_any(haystack: &str, needles: &[&str]) -> Option<u32> {
        let mut best: Option<usize> = None;
        for needle in needles {
            if let Some(idx) = haystack.find(needle) {
                best = Some(match best {
                    Some(current) => current.min(idx),
                    None => idx,
                });
            }
        }
        best.map(|idx| haystack[..idx].chars().filter(|c| *c == '\n').count() as u32 + 1)
    }

    pub fn primary_item_contract_violation(
        file_path: &std::path::Path,
        source: &str,
    ) -> Option<PrimaryItemViolation> {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(err) => {
                return Some(PrimaryItemViolation {
                    message: format!("rust parse failed: {err}"),
                    line: None,
                });
            }
        };

        let mut primary_items: Vec<String> = Vec::new();
        for item in &ast.items {
            match item {
                syn::Item::Struct(s) => primary_items.push(s.ident.to_string()),
                syn::Item::Enum(e) => primary_items.push(e.ident.to_string()),
                syn::Item::Trait(t) => primary_items.push(t.ident.to_string()),
                _ => {}
            }
        }

        let stem = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if primary_items.len() > 1 {
            return Some(PrimaryItemViolation {
                message: "file contains multiple primary struct/enum/trait declarations"
                    .to_string(),
                line: None,
            });
        }

        if primary_items.is_empty() {
            return None;
        }

        let primary_name = &primary_items[0];
        let expected = to_snake_case(primary_name);
        if expected != stem {
            return Some(PrimaryItemViolation {
                message: format!(
                    "primary item name '{primary_name}' does not match file stem '{stem}'"
                ),
                line: None,
            });
        }

        None
    }

    pub fn imports_backend_crate(source: &str, backend_crate_name: &str) -> bool {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => return false,
        };

        for item in ast.items {
            if let syn::Item::Use(item_use) = item
                && use_tree_starts_with(&item_use.tree, backend_crate_name)
            {
                return true;
            }
        }
        false
    }

    pub fn unsafe_signals(source: &str) -> UnsafeSignals {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => {
                return UnsafeSignals {
                    has_unsafe_usage: false,
                };
            }
        };

        let mut visitor = UnsafeVisitor {
            has_unsafe_usage: false,
        };
        syn::visit::Visit::visit_file(&mut visitor, &ast);
        UnsafeSignals {
            has_unsafe_usage: visitor.has_unsafe_usage,
        }
    }

    pub fn underscore_signals(source: &str) -> UnderscoreSignals {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => {
                return UnderscoreSignals {
                    has_wildcard_discard: false,
                    has_prefixed_binding: false,
                };
            }
        };

        let mut visitor = UnderscoreVisitor {
            has_wildcard_discard: false,
            has_prefixed_binding: false,
        };
        syn::visit::Visit::visit_file(&mut visitor, &ast);
        UnderscoreSignals {
            has_wildcard_discard: visitor.has_wildcard_discard,
            has_prefixed_binding: visitor.has_prefixed_binding,
        }
    }

    pub fn stdout_macro_signals(source: &str) -> StdoutMacroSignals {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => {
                return StdoutMacroSignals {
                    has_stdout_macro: false,
                };
            }
        };

        let mut visitor = StdoutMacroVisitor {
            has_stdout_macro: false,
        };
        syn::visit::Visit::visit_file(&mut visitor, &ast);
        StdoutMacroSignals {
            has_stdout_macro: visitor.has_stdout_macro,
        }
    }

    pub fn local_use_statement_lines(source: &str) -> Vec<u32> {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => return Vec::new(),
        };

        let mut visitor = LocalUseVisitor {
            block_depth: 0,
            lines: Vec::new(),
        };
        syn::visit::Visit::visit_file(&mut visitor, &ast);
        visitor.lines
    }

    pub fn inline_test_attribute_lines(source: &str) -> Vec<u32> {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => return Vec::new(),
        };

        let mut visitor = InlineTestAttributeVisitor { lines: Vec::new() };
        syn::visit::Visit::visit_file(&mut visitor, &ast);
        visitor.lines
    }

    pub fn main_module_item_violations(source: &str) -> Vec<MainItemViolation> {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(_) => return Vec::new(),
        };

        let mut out = Vec::new();
        for item in ast.items {
            match item {
                syn::Item::Struct(s) => out.push(MainItemViolation {
                    kind: MainItemViolationKind::Struct,
                    line: line_of_span(s.ident.span()),
                }),
                syn::Item::Enum(e) => out.push(MainItemViolation {
                    kind: MainItemViolationKind::Enum,
                    line: line_of_span(e.ident.span()),
                }),
                syn::Item::Trait(t) => out.push(MainItemViolation {
                    kind: MainItemViolationKind::Trait,
                    line: line_of_span(t.ident.span()),
                }),
                syn::Item::Impl(i) => out.push(MainItemViolation {
                    kind: MainItemViolationKind::Impl,
                    line: line_of_span(i.impl_token.span),
                }),
                syn::Item::Fn(f) => {
                    if f.sig.ident != "main" {
                        out.push(MainItemViolation {
                            kind: MainItemViolationKind::NonEntrypointFn,
                            line: line_of_span(f.sig.ident.span()),
                        });
                    }
                }
                _ => {}
            }
        }
        out
    }
}

fn to_snake_case(input: &str) -> String {
    let mut out = String::new();
    for (idx, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if idx > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn use_tree_starts_with(tree: &syn::UseTree, prefix: &str) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            path.ident == prefix || use_tree_starts_with(&path.tree, prefix)
        }
        syn::UseTree::Name(name) => name.ident == prefix,
        syn::UseTree::Rename(rename) => rename.ident == prefix,
        syn::UseTree::Glob(_) => false,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|child| use_tree_starts_with(child, prefix)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnsafeVisitor {
    has_unsafe_usage: bool,
}

impl<'ast> syn::visit::Visit<'ast> for UnsafeVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if node.sig.unsafety.is_some() {
            self.has_unsafe_usage = true;
        }
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr_unsafe(&mut self, _node: &'ast syn::ExprUnsafe) {
        self.has_unsafe_usage = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnderscoreVisitor {
    has_wildcard_discard: bool,
    has_prefixed_binding: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StdoutMacroVisitor {
    has_stdout_macro: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalUseVisitor {
    block_depth: usize,
    lines: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InlineTestAttributeVisitor {
    lines: Vec<u32>,
}

impl<'ast> syn::visit::Visit<'ast> for UnderscoreVisitor {
    fn visit_local(&mut self, node: &'ast syn::Local) {
        match &node.pat {
            syn::Pat::Wild(_) => {
                self.has_wildcard_discard = true;
            }
            other => {
                if pat_contains_prefixed_binding(other) {
                    self.has_prefixed_binding = true;
                }
            }
        }
        syn::visit::visit_local(self, node);
    }

    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        if node.ident.to_string().starts_with('_') {
            self.has_prefixed_binding = true;
        }
        syn::visit::visit_pat_ident(self, node);
    }
}

impl<'ast> syn::visit::Visit<'ast> for StdoutMacroVisitor {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let is_stdout_macro = node
            .path
            .segments
            .last()
            .map(|segment| {
                matches!(
                    segment.ident.to_string().as_str(),
                    "println" | "print" | "eprint" | "eprintln"
                )
            })
            .unwrap_or(false);
        if is_stdout_macro {
            self.has_stdout_macro = true;
        }
        syn::visit::visit_macro(self, node);
    }
}

impl<'ast> syn::visit::Visit<'ast> for LocalUseVisitor {
    fn visit_block(&mut self, node: &'ast syn::Block) {
        self.block_depth += 1;
        syn::visit::visit_block(self, node);
        self.block_depth = self.block_depth.saturating_sub(1);
    }

    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        if self.block_depth > 0
            && let syn::Stmt::Item(syn::Item::Use(item_use)) = node
            && let Some(line) = line_of_span(item_use.use_token.span)
        {
            self.lines.push(line);
        }
        syn::visit::visit_stmt(self, node);
    }
}

impl<'ast> syn::visit::Visit<'ast> for InlineTestAttributeVisitor {
    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        if is_test_attribute(node)
            && let Some(line) = line_of_span(node.path().span())
        {
            self.lines.push(line);
        }
        syn::visit::visit_attribute(self, node);
    }
}

fn line_of_span(span: proc_macro2::Span) -> Option<u32> {
    let line = span.start().line;
    if line == 0 {
        return None;
    }
    Some(line as u32)
}

fn is_test_attribute(attr: &syn::Attribute) -> bool {
    if attr.path().is_ident("test")
        || attr.path().is_ident("rstest")
        || attr.path().is_ident("test_case")
        || attr.path().is_ident("quickcheck")
    {
        return true;
    }
    if !attr.path().is_ident("cfg") {
        return false;
    }
    match &attr.meta {
        syn::Meta::List(list) => list.tokens.to_string().replace(' ', "") == "test",
        _ => false,
    }
}

fn pat_contains_prefixed_binding(pat: &syn::Pat) -> bool {
    match pat {
        syn::Pat::Ident(ident) => ident.ident.to_string().starts_with('_'),
        syn::Pat::Tuple(tuple) => tuple.elems.iter().any(pat_contains_prefixed_binding),
        syn::Pat::TupleStruct(tuple_struct) => {
            tuple_struct.elems.iter().any(pat_contains_prefixed_binding)
        }
        syn::Pat::Struct(struct_pat) => struct_pat
            .fields
            .iter()
            .any(|field| pat_contains_prefixed_binding(&field.pat)),
        syn::Pat::Slice(slice) => slice.elems.iter().any(pat_contains_prefixed_binding),
        syn::Pat::Reference(reference) => pat_contains_prefixed_binding(&reference.pat),
        syn::Pat::Type(pat_type) => pat_contains_prefixed_binding(&pat_type.pat),
        syn::Pat::Or(pat_or) => pat_or.cases.iter().any(pat_contains_prefixed_binding),
        syn::Pat::Paren(pat_paren) => pat_contains_prefixed_binding(&pat_paren.pat),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{MainItemViolationKind, RustParser};

    #[test]
    fn stdout_macro_signal_ignores_string_literals() {
        let source = r#"
            fn demo() {
                let text = "println!(\"debug\")";
                let _copy = text.to_string();
            }
        "#;
        let signals = RustParser::stdout_macro_signals(source);
        assert!(!signals.has_stdout_macro);
    }

    #[test]
    fn stdout_macro_signal_detects_real_invocation() {
        let source = r#"
            fn demo() {
                println!("hello");
            }
        "#;
        let signals = RustParser::stdout_macro_signals(source);
        assert!(signals.has_stdout_macro);
    }

    #[test]
    fn local_use_lines_detects_only_block_scope_uses() {
        let source = r#"
            use std::fmt::Debug;

            fn demo() {
                use std::collections::HashMap;
                let _m: HashMap<String, String> = HashMap::new();
            }
        "#;
        let lines = RustParser::local_use_statement_lines(source);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn inline_test_attribute_lines_detects_cfg_and_test_attrs() {
        let source = r#"
            #[cfg(test)]
            mod tests {
                #[test]
                fn smoke() {}
            }
        "#;
        let lines = RustParser::inline_test_attribute_lines(source);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn main_module_item_violations_detect_disallowed_top_level_items() {
        let source = r#"
            struct App;
            enum Mode { Fast }
            trait Runner {}
            impl App { fn run(&self) {} }
            fn helper() {}
            fn main() {}
        "#;
        let violations = RustParser::main_module_item_violations(source);
        assert!(
            violations
                .iter()
                .any(|v| v.kind == MainItemViolationKind::Struct)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.kind == MainItemViolationKind::Enum)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.kind == MainItemViolationKind::Trait)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.kind == MainItemViolationKind::Impl)
        );
        assert!(
            violations
                .iter()
                .any(|v| v.kind == MainItemViolationKind::NonEntrypointFn)
        );
    }
}
