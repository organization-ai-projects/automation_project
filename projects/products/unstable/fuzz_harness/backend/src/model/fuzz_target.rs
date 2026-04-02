use crate::model::FuzzInput;
use crate::model::FuzzResult;

pub(crate) trait FuzzTarget {
    fn name(&self) -> &str;
    fn execute(&self, input: &FuzzInput) -> FuzzResult;
}
