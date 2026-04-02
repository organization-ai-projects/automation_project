//! projects/products/unstable/rust_language/backend/src/engine/rhl_engine.rs
use crate::ai_assist::{ErrorAnalyzer, TranspileValidator};
use crate::compiler::{Lexer, Parser, Transpiler};
use crate::engine::binary_encoder::BinaryEncoder;
use crate::engine::engine_errors::EngineErrors;
use crate::engine::ron_loader::RonLoader;
use crate::model::{BinaryFormat, ProjectConfig, RhlAst, SourceFile};

use std::path::Path;

pub(crate) struct RhlEngine {
    config: ProjectConfig,
}

impl RhlEngine {
    pub(crate) fn from_ron(config_path: &Path) -> Result<Self, EngineErrors> {
        let config = RonLoader::load_config(config_path)
            .map_err(|e| EngineErrors::Runtime(e.to_string()))?;
        Ok(Self { config })
    }

    pub(crate) fn from_config(config: ProjectConfig) -> Self {
        Self { config }
    }

    pub(crate) fn compile_source(&self, source: &SourceFile) -> Result<String, EngineErrors> {
        if !source.is_rhl() {
            return Err(EngineErrors::Runtime(format!(
                "file {} does not have .rhl extension",
                source.path
            )));
        }
        let mut lexer = Lexer::new(&source.content);
        let tokens = lexer
            .tokenize()
            .map_err(|e| EngineErrors::Runtime(e.to_string()))?;
        let mut parser = Parser::new(tokens);
        let ast = parser
            .parse()
            .map_err(|e| EngineErrors::Runtime(e.to_string()))?;
        Transpiler::transpile(&ast).map_err(|e| EngineErrors::Runtime(e.to_string()))
    }

    pub(crate) fn compile_to_binary(
        &self,
        source: &SourceFile,
    ) -> Result<BinaryFormat, EngineErrors> {
        let rust_code = self.compile_source(source)?;
        BinaryEncoder::encode_rust_to_binary(&rust_code)
            .map_err(|e| EngineErrors::Runtime(e.to_string()))
    }

    pub(crate) fn compile_string(&self, source_code: &str) -> Result<String, EngineErrors> {
        let file = SourceFile::new("inline.rhl".into(), source_code.into());
        self.compile_source(&file)
    }

    pub(crate) fn save_config(&self, path: &Path) -> Result<(), EngineErrors> {
        RonLoader::save_config(path, &self.config).map_err(|e| EngineErrors::Runtime(e.to_string()))
    }

    pub(crate) fn save_binary(
        &self,
        source: &SourceFile,
        output_path: &Path,
    ) -> Result<(), EngineErrors> {
        let binary = self.compile_to_binary(source)?;
        BinaryEncoder::write_binary(output_path, &binary)
            .map_err(|e| EngineErrors::Runtime(e.to_string()))
    }

    pub(crate) fn parse_to_ast(&self, source_code: &str) -> Result<RhlAst, EngineErrors> {
        let mut lexer = Lexer::new(source_code);
        let tokens = lexer
            .tokenize()
            .map_err(|e| EngineErrors::Runtime(e.to_string()))?;
        let mut parser = Parser::new(tokens);
        parser
            .parse()
            .map_err(|e| EngineErrors::Runtime(e.to_string()))
    }

    pub(crate) fn config(&self) -> &ProjectConfig {
        &self.config
    }

    pub(crate) fn check_source(file_path: &str, content: &str) -> Result<(), EngineErrors> {
        let source = SourceFile::new(file_path.to_string(), content.to_string());

        let config = ProjectConfig::new("inline".into(), "0.1.0".into(), file_path.to_string());
        let engine = RhlEngine::from_config(config);

        match engine.compile_source(&source) {
            Ok(_) => {
                let mut validator = TranspileValidator::new()?;
                validator.validate_transpilation(content, "")?;
            }
            Err(e) => {
                let mut analyzer = ErrorAnalyzer::new()?;
                analyzer.analyze_compilation_error(content, &e.to_string())?;
                analyzer.suggest_fix(content, &e.to_string())?;
            }
        }

        Ok(())
    }
}
