/// Extracted symbol information from source code.
pub struct SymbolTable {
    /// (name, line) pairs for definitions (let/const/fn).
    pub definitions: Vec<(String, usize)>,
    /// (name, line) pairs for references.
    pub references: Vec<(String, usize)>,
}

/// Rust keywords and common macro names to exclude from reference tracking.
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "become", "box", "break", "const", "continue", "crate", "do", "dyn",
    "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return", "self",
    "Self", "static", "struct", "super", "trait", "true", "type", "typeof", "union", "unsafe",
    "unsized", "use", "virtual", "where", "while", "yield",
];

/// Extracts symbols (definitions and references) from Rust-like source text.
pub struct SymbolExtractor;

impl SymbolExtractor {
    pub fn extract(source: &str) -> SymbolTable {
        let mut definitions = Vec::new();
        let mut references = Vec::new();

        for (idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            let line_num = idx + 1;

            // Detect `let <name>`, `const <name>`, `fn <name>`
            for keyword in &["let ", "const ", "fn "] {
                if let Some(rest) = trimmed.strip_prefix(keyword) {
                    let rest = rest.trim_start_matches("mut ");
                    if let Some(name) = rest
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                    {
                        let name = name.trim();
                        if !name.is_empty() {
                            definitions.push((name.to_string(), line_num));
                        }
                    }
                }
            }

            // Record all identifier-like tokens as references so that forward
            // references and undefined-symbol detection work correctly.
            if !trimmed.starts_with("let ")
                && !trimmed.starts_with("const ")
                && !trimmed.starts_with("fn ")
                && !trimmed.starts_with("//")
            {
                for word in trimmed.split(|c: char| !c.is_alphanumeric() && c != '_') {
                    let word = word.trim();
                    if !word.is_empty()
                        && word
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_alphabetic() || c == '_')
                        && !RUST_KEYWORDS.contains(&word)
                    {
                        references.push((word.to_string(), line_num));
                    }
                }
            }
        }

        SymbolTable {
            definitions,
            references,
        }
    }
}
