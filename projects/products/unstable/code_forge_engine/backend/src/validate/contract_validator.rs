use crate::contract::contract::Contract;
use crate::diagnostics::backend_error::BackendError;

pub struct ContractValidator;

impl ContractValidator {
    pub fn validate(contract: &Contract) -> Result<(), BackendError> {
        if contract.name.trim().is_empty() {
            return Err(BackendError::Validation(
                "contract name must not be empty".to_string(),
            ));
        }
        if contract.version.trim().is_empty() {
            return Err(BackendError::Validation(
                "contract version must not be empty".to_string(),
            ));
        }
        if contract.modules.is_empty() {
            return Err(BackendError::Validation(
                "contract must declare at least one module".to_string(),
            ));
        }
        Ok(())
    }
}
