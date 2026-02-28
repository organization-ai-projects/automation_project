// projects/products/unstable/code_forge_engine/backend/src/validate/contract_validator.rs
use crate::contract::contract::Contract;
use crate::diagnostics::error::ForgeError;

pub struct ContractValidator;

impl ContractValidator {
    pub fn validate(contract: &Contract) -> Result<(), ForgeError> {
        if contract.name.is_empty() {
            return Err(ForgeError::Validation("contract name must not be empty".to_string()));
        }
        Ok(())
    }
}
