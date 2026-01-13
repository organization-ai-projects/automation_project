// symbolic/src/validator.rs
use crate::validator::ValidationError;
use crate::validator::validation_result::ValidationResult;
use common::common_id::CommonID;
use common::custom_uuid::Id128;

/// Validateur de code Rust
pub struct CodeValidator {
    strict_mode: bool,
}

impl CodeValidator {
    pub fn new() -> Result<Self, ValidationError> {
        Ok(Self { strict_mode: false })
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Valide du code Rust
    pub fn validate(&self, code: &str) -> Result<ValidationResult, ValidationError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validation de base
        if !CommonID::is_valid(Id128::new(0, None, None)) {
            return Ok(ValidationResult::invalid(vec!["Invalid ID".to_string()]));
        }

        // Vérifier la syntaxe avec syn
        match syn::parse_file(code) {
            Ok(syntax_tree) => {
                // Validation réussie au niveau syntaxe
                println!("Code parsed successfully");

                // Validations sémantiques supplémentaires
                self.validate_semantics(&syntax_tree, &mut warnings);
            }
            Err(e) => {
                errors.push(format!("Syntax error: {}", e));
                return Ok(ValidationResult::invalid(errors));
            }
        }

        // Invalider explicitement le code vide après la validation syntaxique
        if code.trim().is_empty() {
            errors.push("Code is empty".to_string());
            return Ok(ValidationResult::invalid(errors));
        }

        // Vérifications additionnelles
        self.check_common_issues(code, &mut warnings);

        if errors.is_empty() {
            Ok(ValidationResult::valid().with_warnings(warnings))
        } else {
            Ok(ValidationResult::invalid(errors).with_warnings(warnings))
        }
    }

    /// Valide la syntaxe uniquement (plus rapide)
    pub fn validate_syntax(&self, code: &str) -> Result<ValidationResult, ValidationError> {
        if code.trim().is_empty() {
            return Ok(ValidationResult::invalid(vec!["Code is empty".to_string()]));
        }

        match syn::parse_file(code) {
            Ok(_) => Ok(ValidationResult::valid()),
            Err(e) => Ok(ValidationResult::invalid(vec![format!(
                "Syntax error: {}",
                e
            )])),
        }
    }

    /// Suggère un fix pour des erreurs communes
    pub fn suggest_fix(&self, code: &str, errors: &[String]) -> Option<String> {
        // Si erreur de missing semicolon
        if errors.iter().any(|e| e.contains("expected `;`")) {
            return self.try_add_semicolons(code);
        }

        // Si erreur de parenthèses non fermées
        if errors.iter().any(|e| e.contains("unclosed delimiter")) {
            return self.try_close_delimiters(code);
        }

        // Si erreur de type struct/enum mal formé
        if errors.iter().any(|e| e.contains("expected `{`")) {
            return self.try_fix_struct_format(code);
        }

        None
    }

    /// Validations sémantiques supplémentaires
    fn validate_semantics(&self, _syntax_tree: &syn::File, warnings: &mut Vec<String>) {
        // TODO: Ajouter des validations sémantiques plus avancées
        // - Vérifier les imports inutilisés
        // - Vérifier les variables non utilisées
        // - Vérifier les types incohérents

        if self.strict_mode {
            warnings.push("Strict mode enabled: additional checks not yet implemented".to_string());
        }
    }

    /// Vérifie les problèmes courants
    fn check_common_issues(&self, code: &str, warnings: &mut Vec<String>) {
        // Vérifier les println! en production
        if code.contains("println!") {
            warnings.push("Code contains println! statements".to_string());
        }

        // Vérifier les todo!()
        if code.contains("todo!()") {
            warnings.push("Code contains todo!() macros".to_string());
        }

        // Vérifier les unwrap()
        if code.contains(".unwrap()") {
            warnings
                .push("Code contains .unwrap() calls (consider proper error handling)".to_string());
        }

        // Vérifier les #[allow(dead_code)]
        if code.contains("#[allow(dead_code)]") {
            warnings.push("Code contains #[allow(dead_code)] attributes".to_string());
        }

        // Vérifier la longueur des lignes
        for (i, line) in code.lines().enumerate() {
            if line.len() > 100 {
                warnings.push(format!("Line {} exceeds 100 characters", i + 1));
            }
        }
    }

    /// Tente d'ajouter des points-virgules manquants
    fn try_add_semicolons(&self, code: &str) -> Option<String> {
        let mut fixed = String::new();

        for line in code.lines() {
            let trimmed = line.trim_end();
            if !trimmed.is_empty()
                && !trimmed.ends_with(';')
                && !trimmed.ends_with('{')
                && !trimmed.ends_with('}')
                && !trimmed.ends_with(',')
            {
                fixed.push_str(trimmed);
                fixed.push(';');
                fixed.push('\n');
            } else {
                fixed.push_str(line);
                fixed.push('\n');
            }
        }

        // Vérifier si le code corrigé est valide
        // Encapsuler le code dans une fonction pour validation
        let wrapped_code = format!("fn test_wrapper() {{\n{}\n}}", fixed);
        if !self.validate_code(&wrapped_code) {
            println!("[DEBUG] Validation failed for wrapped code");
            return None;
        }

        println!("[DEBUG] Code before fix: {}", code);
        println!("[DEBUG] Code after fix: {}", fixed);
        Some(fixed)
    }

    /// Tente de fermer les délimiteurs ouverts
    fn try_close_delimiters(&self, code: &str) -> Option<String> {
        let mut open_braces = 0;
        let mut open_brackets = 0;
        let mut open_parens = 0;

        for ch in code.chars() {
            match ch {
                '{' => open_braces += 1,
                '}' => open_braces -= 1,
                '[' => open_brackets += 1,
                ']' => open_brackets -= 1,
                '(' => open_parens += 1,
                ')' => open_parens -= 1,
                _ => {}
            }
        }

        let mut fixed = code.to_string();

        // Fermer les délimiteurs ouverts
        for _ in 0..open_parens.max(0) {
            fixed.push(')');
        }
        for _ in 0..open_brackets.max(0) {
            fixed.push(']');
        }
        for _ in 0..open_braces.max(0) {
            fixed.push('}');
        }

        // Vérifier si le fix fonctionne
        if syn::parse_file(&fixed).is_ok() {
            Some(fixed)
        } else {
            None
        }
    }

    /// Tente de corriger le format d'un struct/enum
    fn try_fix_struct_format(&self, code: &str) -> Option<String> {
        // Correction simple: ajouter {} si manquant après struct/enum
        let mut fixed = code.to_string();

        if code.contains("struct") && !code.contains('{') {
            fixed.push_str("struct {}");
        }

        if code.contains("enum") && !code.contains('{') {
            fixed.push_str("enum {}");
        }

        // Vérifier si le fix fonctionne
        if syn::parse_file(&fixed).is_ok() {
            Some(fixed)
        } else {
            None
        }
    }

    /// Vérifie si le code est valide (sans erreurs de syntaxe)
    fn validate_code(&self, code: &str) -> bool {
        syn::parse_file(code).is_ok()
    }
}

impl Default for CodeValidator {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = CodeValidator::new();
        assert!(validator.is_ok());
    }

    #[test]
    fn test_valid_code() {
        let validator = CodeValidator::new().unwrap();
        let code = r#"
            fn main() {
                println!("Hello, world!");
            }
        "#;

        let result = validator.validate(code);
        assert!(result.is_ok());
        assert!(result.unwrap().is_valid);
    }

    #[test]
    fn test_invalid_syntax() {
        let validator = CodeValidator::new().unwrap();
        let code = "fn main( {"; // Missing closing paren

        let result = validator.validate(code);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_empty_code() {
        let validator = CodeValidator::new().unwrap();
        let result = validator.validate("");

        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(!validation.is_valid);
    }

    #[test]
    fn test_warnings() {
        let validator = CodeValidator::new().unwrap();
        let code = r#"
            fn main() {
                println!("test");
                let x = Some(5).unwrap();
                todo!();
            }
        "#;

        let result = validator.validate(code);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_valid);
        assert!(!validation.warnings.is_empty());
        assert!(validation.warnings.iter().any(|w| w.contains("println!")));
        assert!(validation.warnings.iter().any(|w| w.contains("unwrap()")));
        assert!(validation.warnings.iter().any(|w| w.contains("todo!()")));
    }

    #[test]
    fn test_suggest_fix_semicolon() {
        let validator = CodeValidator::new().unwrap();
        let code = "let x = 5";

        let errors = vec!["expected `;`".to_string()];
        let fix = validator.suggest_fix(code, &errors);

        assert!(fix.is_some());
        assert!(fix.unwrap().contains(';'));
    }

    #[test]
    fn test_suggest_fix_delimiters() {
        let validator = CodeValidator::new().unwrap();
        let code = "fn main() { println!(\"test\"";

        let errors = vec!["unclosed delimiter".to_string()];
        let fix = validator.suggest_fix(code, &errors);

        assert!(fix.is_some());
    }

    #[test]
    fn test_strict_mode() {
        let validator = CodeValidator::new().unwrap().with_strict_mode(true);
        assert!(validator.strict_mode);
    }

    #[test]
    fn test_validate_syntax_only() {
        let validator = CodeValidator::new().unwrap();
        let code = "fn test() {}";

        let result = validator.validate_syntax(code);
        assert!(result.is_ok());
        assert!(result.unwrap().is_valid);
    }
}
