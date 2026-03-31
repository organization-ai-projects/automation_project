/// Extracted symbol information from source code.
pub struct SymbolTable {
    /// (name, line) pairs for definitions (let/const/fn).
    pub definitions: Vec<(String, usize)>,
    /// (name, line) pairs for references.
    pub references: Vec<(String, usize)>,
}

/// Extracts symbols (definitions and references) from Rust-like source text.
pub struct SymbolExtractor;

impl SymbolExtractor {
    pub fn extract(source: &str) -> SymbolTable {
        let mut definitions = Vec::new();
        let mut references = Vec::new();
        let mut defined_names: Vec<String> = Vec::new();

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
                            defined_names.push(name.to_string());
                        }
                    }
                }
            }

            // Simple reference detection: identifiers used outside of definitions.
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
                        && defined_names.contains(&word.to_string())
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
