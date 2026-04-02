use crate::model::{FuzzInput, FuzzResult, FuzzTarget};

pub(crate) struct DummyTarget;

impl FuzzTarget for DummyTarget {
    fn name(&self) -> &str {
        "dummy"
    }

    fn execute(&self, input: &FuzzInput) -> FuzzResult {
        for window in input.data.windows(2) {
            if window[0] >= 0x80 && window[1] >= 0x80 {
                return FuzzResult::Fail(format!(
                    "consecutive high bytes at pattern [{:#04x}, {:#04x}]",
                    window[0], window[1]
                ));
            }
        }
        FuzzResult::Pass
    }
}
