use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::diagnostics::error::Error;
use crate::engine::binary_encoder::BinaryEncoder;
use crate::engine::ron_loader::RonLoader;
use crate::model::binary_format::BinaryFormat;
use crate::model::project_config::ProjectConfig;
use crate::model::rhl_ast::RhlAst;
use crate::model::source_file::SourceFile;

use std::path::Path;

pub struct RhlEngine {
    config: ProjectConfig,
}

impl RhlEngine {
    pub fn from_ron(config_path: &Path) -> Result<Self, Error> {
        let config = RonLoader::load_config(config_path)?;
        Ok(Self { config })
    }

    pub fn from_config(config: ProjectConfig) -> Self {
        Self { config }
    }

    pub fn compile_source(&self, source: &SourceFile) -> Result<String, Error> {
        if !source.is_rhl() {
            return Err(Error::Transpilation(format!(
                "file {} does not have .rhl extension",
                source.path
            )));
        }
        let mut lexer = Lexer::new(&source.content);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        Transpiler::transpile(&ast)
    }

    pub fn compile_to_binary(&self, source: &SourceFile) -> Result<BinaryFormat, Error> {
        let rust_code = self.compile_source(source)?;
        BinaryEncoder::encode_ast_to_binary(&rust_code)
    }

    pub fn compile_string(&self, source_code: &str) -> Result<String, Error> {
        let file = SourceFile::new("inline.rhl".into(), source_code.into());
        self.compile_source(&file)
    }

    pub fn save_config(&self, path: &Path) -> Result<(), Error> {
        RonLoader::save_config(path, &self.config)?;
        Ok(())
    }

    pub fn save_binary(&self, source: &SourceFile, output_path: &Path) -> Result<(), Error> {
        let binary = self.compile_to_binary(source)?;
        BinaryEncoder::write_binary(output_path, &binary)?;
        Ok(())
    }

    pub fn parse_to_ast(&self, source_code: &str) -> Result<RhlAst, Error> {
        let mut lexer = Lexer::new(source_code);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }
}
