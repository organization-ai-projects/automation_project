// projects/libraries/symbolic/src/rules/rules_engine.rs
use regex::Regex;
use std::collections::HashMap;

use crate::rules::{
    CodeTemplate, RefactoringRule, RulesError, refactoring_result::RefactoringResult,
};

/// Moteur de règles pour génération symbolique
pub struct RulesEngine {
    templates: HashMap<String, Vec<CodeTemplate>>,
    refactoring_rules: Vec<RefactoringRule>,
}

impl RulesEngine {
    pub fn new() -> Result<Self, RulesError> {
        let mut engine = Self {
            templates: HashMap::new(),
            refactoring_rules: Vec::new(),
        };

        // Initialiser avec des templates de base
        engine.init_default_templates()?;
        engine.init_refactoring_rules()?;

        Ok(engine)
    }

    /// Initialise les templates par défaut
    fn init_default_templates(&mut self) -> Result<(), RulesError> {
        // Template: struct simple
        self.add_template(
            "struct",
            vec!["create struct", "new struct", "define struct"],
            r#"#[derive(Debug, Clone)]
pub struct {name} {{
{fields}
}}"#,
            0.9,
        )?;

        // Template: enum
        self.add_template(
            "enum",
            vec!["create enum", "new enum", "define enum"],
            r#"#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum {name} {{
{variants}
}}"#,
            0.9,
        )?;

        // Template: fonction simple
        self.add_template(
            "function",
            vec!["create function", "new function", "define function"],
            r#"pub fn {name}() {{
    todo!()
}}"#,
            0.9,
        )?;

        // Template: impl block
        self.add_template(
            "impl",
            vec!["implement", "impl block"],
            r#"impl {name} {{
    pub fn new({params}) -> Self {{
        Self {{
{fields}
        }}
    }}
}}"#,
            0.8,
        )?;

        // Template: trait
        self.add_template(
            "trait",
            vec!["create trait", "define trait"],
            r#"pub trait {name} {{
{methods}
}}"#,
            0.85,
        )?;

        // Template: fonction de calcul spécifique
        self.add_template(
            "function",
            vec!["create function calculate", "new function calculate"],
            r#"pub fn calculate() {{
    todo!()
}}"#,
            0.9,
        )?;

        Ok(())
    }

    /// Initialise les règles de refactoring
    fn init_refactoring_rules(&mut self) -> Result<(), RulesError> {
        self.refactoring_rules.push(RefactoringRule {
            name: "add_debug_derive".to_string(),
            pattern: r"^(pub )?struct ".to_string(),
            replacement: r"#[derive(Debug)]\n$0".to_string(),
            description: "Add Debug derive to structs".to_string(),
        });

        self.refactoring_rules.push(RefactoringRule {
            name: "add_clone_derive".to_string(),
            pattern: r"^(pub )?struct ".to_string(),
            replacement: r"#[derive(Clone)]\n$0".to_string(),
            description: "Add Clone derive to structs".to_string(),
        });

        self.refactoring_rules.push(RefactoringRule {
            name: "make_public".to_string(),
            pattern: r"^struct ".to_string(),
            replacement: r"pub struct ".to_string(),
            description: "Make struct public".to_string(),
        });

        self.refactoring_rules.push(RefactoringRule {
            name: "make_fields_public".to_string(),
            pattern: r"    ([a-z_][a-z0-9_]*): ".to_string(),
            replacement: r"    pub $1: ".to_string(),
            description: "Make struct fields public".to_string(),
        });

        Ok(())
    }

    /// Ajoute un template
    fn add_template(
        &mut self,
        category: &str,
        patterns: Vec<&str>,
        template: &str,
        confidence: f64,
    ) -> Result<(), RulesError> {
        let templates = self.templates.entry(category.to_string()).or_default();

        for pattern in patterns {
            templates.push(CodeTemplate {
                pattern: pattern.to_string(),
                template: template.to_string(),
                confidence,
            });
        }

        Ok(())
    }

    /// Génère du code à partir d'un prompt et d'un template
    pub fn generate(&self, prompt: &str, context: Option<&str>) -> Result<String, RulesError> {
        let prompt_lower = prompt.to_lowercase();

        println!("[DEBUG] Generating code for prompt: {}", prompt);

        // Trouver le template qui match
        for (category, templates) in &self.templates {
            for template in templates {
                if prompt_lower.contains(&template.pattern) {
                    return self.fill_template(&template.template, prompt, context, Some(category));
                }
            }
        }

        Err(RulesError::TemplateNotFound(format!(
            "No template found for prompt: {}",
            prompt
        )))
    }

    /// Remplit un template avec les données extraites du prompt
    fn fill_template(
        &self,
        template: &str,
        prompt: &str,
        context: Option<&str>,
        category: Option<&str>,
    ) -> Result<String, RulesError> {
        let mut result = template.to_string();

        // Parser le prompt pour extraire les informations
        let parsed = self.parse_prompt(prompt, context, category)?;

        // Remplacer les placeholders sans modifier la casse
        for (key, value) in parsed {
            result = result.replace(&format!("{{{}}}", key), &value);
        }

        Ok(result)
    }

    /// Parse un prompt pour extraire les informations structurées
    fn parse_prompt(
        &self,
        prompt: &str,
        context: Option<&str>,
        category: Option<&str>,
    ) -> Result<HashMap<String, String>, RulesError> {
        let mut data = HashMap::new();

        // Extraire le nom (chercher un mot capitalisé)
        if let Some(name) = self.extract_name(prompt) {
            data.insert("name".to_string(), name);
        } else {
            data.insert("name".to_string(), "MyType".to_string());
        }

        // Extraire les fields si présents
        if let Some(fields) = self.extract_fields(prompt, context) {
            data.insert("fields".to_string(), fields);
        } else {
            data.insert("fields".to_string(), "    // TODO: Add fields".to_string());
        }

        // Extraire les variants pour enum
        if let Some(variants) = self.extract_variants(prompt, context) {
            data.insert("variants".to_string(), variants);
        } else {
            data.insert(
                "variants".to_string(),
                "    Variant1,\n    Variant2,".to_string(),
            );
        }

        // Ajouter la catégorie si disponible
        if let Some(cat) = category {
            data.insert("category".to_string(), cat.to_string());
        }

        // Paramètres de fonction
        data.insert("params".to_string(), "".to_string());
        data.insert("return_type".to_string(), "()".to_string());

        // Methods pour trait
        data.insert(
            "methods".to_string(),
            "    // TODO: Add methods".to_string(),
        );

        Ok(data)
    }

    /// Extrait le nom d'un type depuis le prompt
    fn extract_name(&self, prompt: &str) -> Option<String> {
        // Chercher un mot capitalisé après "struct", "enum", etc.
        let words: Vec<&str> = prompt.split_whitespace().collect();

        println!("[DEBUG] Extracting name from prompt: {}", prompt);
        println!("[DEBUG] Words in prompt: {:?}", words);

        for (i, word) in words.iter().enumerate() {
            if ["struct", "enum", "trait", "function", "fn"].contains(&word.to_lowercase().as_str())
                && let Some(next_word) = words.get(i + 1)
            {
                let name = next_word.to_string();
                println!("[DEBUG] Extracted name: {}", name);
                return Some(name);
            }
        }

        // Fallback: chercher n'importe quel mot capitalisé
        for word in words {
            if word.chars().next()?.is_uppercase() {
                return Some(word.to_string());
            }
        }

        None
    }

    /// Extrait les fields d'un struct depuis le prompt
    fn extract_fields(&self, prompt: &str, context: Option<&str>) -> Option<String> {
        // Chercher "with X and Y" ou "with X, Y"
        if let Some(start) = prompt.find("with ") {
            let fields_text = &prompt[start + 5..];
            let fields: Vec<String> = fields_text
                .split([',', ' '])
                .filter(|s| !s.is_empty() && s != &"and")
                .map(|field| {
                    let field_name = field.trim();
                    format!("    pub {}: String,", field_name)
                })
                .collect();

            if !fields.is_empty() {
                return Some(fields.join("\n"));
            }
        }

        // Fallback amélioré : fournir un champ par défaut
        if let Some(ctx) = context {
            return Some(format!("    // Context: {}", ctx));
        }

        Some("    pub field1: String,\n    pub field2: String,".to_string())
    }

    /// Extrait les variants d'un enum depuis le prompt
    fn extract_variants(&self, prompt: &str, context: Option<&str>) -> Option<String> {
        // Chercher "variants: X, Y, Z"
        if let Some(start) = prompt.find("variants:") {
            let variants_text = &prompt[start + 9..];
            let variants: Vec<String> = variants_text
                .split(',')
                .map(|v| {
                    let variant = v.trim();
                    let capitalized = variant
                        .chars()
                        .next()
                        .map(|c| c.to_uppercase().to_string() + &variant[1..])
                        .unwrap_or_else(|| variant.to_string());
                    format!("    {},", capitalized)
                })
                .collect();

            if !variants.is_empty() {
                return Some(variants.join("\n"));
            }
        }

        // Fallback amélioré : fournir des variantes par défaut
        if let Some(ctx) = context {
            return Some(format!("    // Context: {}", ctx));
        }

        Some("    Variant1,\n    Variant2,".to_string())
    }

    /// Calcule la confiance du match pour un prompt
    pub fn match_confidence(&self, prompt: &str) -> f64 {
        let prompt_lower = prompt.to_lowercase();

        for templates in self.templates.values() {
            for template in templates {
                if prompt_lower.contains(&template.pattern) {
                    return template.confidence;
                }
            }
        }

        0.0
    }

    /// Applique un refactoring au code
    pub fn apply_refactoring(
        &self,
        code: &str,
        instruction: &str,
    ) -> Result<RefactoringResult, RulesError> {
        let instruction_lower = instruction.to_lowercase();
        let mut result_code = code.to_string();
        let mut changes = Vec::new();

        // Chercher les règles applicables
        for rule in &self.refactoring_rules {
            if instruction_lower.contains(&rule.name.replace('_', " ")) {
                // Appliquer la règle (regex avancée)
                let re = Regex::new(&rule.pattern)
                    .map_err(|e| RulesError::InvalidPattern(e.to_string()))?;
                if re.is_match(&result_code) {
                    result_code = re.replace(&result_code, &rule.replacement).to_string();
                    changes.push(rule.description.clone());
                }

                println!("[DEBUG] Applying rule: {}", rule.name);
            }
        }

        println!("[DEBUG] Code before refactoring: {}", code);
        println!("[DEBUG] Instruction: {}", instruction);
        println!("[DEBUG] Code after refactoring: {}", result_code);

        if changes.is_empty() {
            return Err(RulesError::GenerationFailed(format!(
                "No applicable refactoring rules for: {}",
                instruction
            )));
        }

        Ok(RefactoringResult {
            code: result_code,
            confidence: 0.85,
            changes_applied: changes,
        })
    }

    /// Ajoute une règle de refactoring personnalisée
    pub fn add_refactoring_rule(
        &mut self,
        name: String,
        pattern: String,
        replacement: String,
        description: String,
    ) {
        self.refactoring_rules.push(RefactoringRule {
            name,
            pattern,
            replacement,
            description,
        });
    }
}
