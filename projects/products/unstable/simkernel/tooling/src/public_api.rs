#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;
use crate::generate::pack_generator;
use crate::validate::contract_validator;
use std::path::Path;

pub struct PackGenerator;
impl PackGenerator {
    pub fn generate(pack_name: &str, out_dir: &Path) -> Result<(), ToolingError> {
        pack_generator::generate_pack(pack_name, out_dir)
    }
}

pub struct ContractValidator;
impl ContractValidator {
    pub fn validate(path: &Path) -> Result<(), ToolingError> {
        contract_validator::validate_contract(path)
    }
}
