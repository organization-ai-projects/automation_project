mod dummy_target;

pub(crate) use dummy_target::DummyTarget;

use crate::diagnostics::FuzzHarnessError;
use crate::model::FuzzTarget;

pub(crate) fn resolve_target(name: &str) -> Result<Box<dyn FuzzTarget>, FuzzHarnessError> {
    match name {
        "dummy" => Ok(Box::new(DummyTarget)),
        _ => Err(FuzzHarnessError::UnknownTarget(name.to_string())),
    }
}
