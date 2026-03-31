use crate::model::{FuzzInput, FuzzResult, FuzzTarget};
use crate::replay::ReplayFile;
use crate::shrinker::InputShrinker;

struct AlwaysFailTarget;

impl FuzzTarget for AlwaysFailTarget {
    fn name(&self) -> &str {
        "always_fail"
    }

    fn execute(&self, _input: &FuzzInput) -> FuzzResult {
        FuzzResult::Fail("always fails".to_string())
    }
}

struct FailOnHighByteTarget;

impl FuzzTarget for FailOnHighByteTarget {
    fn name(&self) -> &str {
        "fail_on_high_byte"
    }

    fn execute(&self, input: &FuzzInput) -> FuzzResult {
        if input.data.iter().any(|&b| b >= 0x80) {
            FuzzResult::Fail("found high byte".to_string())
        } else {
            FuzzResult::Pass
        }
    }
}

#[test]
fn shrinker_is_deterministic() {
    let replay = ReplayFile {
        target_name: "always_fail".to_string(),
        seed: 42,
        input: FuzzInput {
            data: vec![10, 20, 30, 40, 50],
            index: 0,
        },
        failure_message: "always fails".to_string(),
    };

    let report_a = InputShrinker::shrink(&AlwaysFailTarget, &replay).unwrap();
    let report_b = InputShrinker::shrink(&AlwaysFailTarget, &replay).unwrap();

    assert_eq!(report_a.shrunk_input.data, report_b.shrunk_input.data);
    assert_eq!(report_a.shrink_steps, report_b.shrink_steps);
    assert_eq!(report_a.failure_message, report_b.failure_message);
}

#[test]
fn shrinker_reduces_input_size() {
    let replay = ReplayFile {
        target_name: "always_fail".to_string(),
        seed: 42,
        input: FuzzInput {
            data: vec![10, 20, 30, 40, 50, 60, 70, 80],
            index: 0,
        },
        failure_message: "always fails".to_string(),
    };

    let report = InputShrinker::shrink(&AlwaysFailTarget, &replay).unwrap();
    assert!(
        report.shrunk_input.data.len() <= replay.input.data.len(),
        "shrunk size {} should be <= original size {}",
        report.shrunk_input.data.len(),
        replay.input.data.len()
    );
}

#[test]
fn shrinker_preserves_failure() {
    let replay = ReplayFile {
        target_name: "fail_on_high_byte".to_string(),
        seed: 42,
        input: FuzzInput {
            data: vec![0x10, 0x90, 0x20, 0xA0, 0x30],
            index: 0,
        },
        failure_message: "found high byte".to_string(),
    };

    let report = InputShrinker::shrink(&FailOnHighByteTarget, &replay).unwrap();
    let result = FailOnHighByteTarget.execute(&FuzzInput {
        data: report.shrunk_input.data.clone(),
        index: 0,
    });
    match result {
        FuzzResult::Fail(_) => {}
        FuzzResult::Pass => panic!("shrunk input should still fail"),
    }
}

#[test]
fn shrinker_reduces_byte_values() {
    let replay = ReplayFile {
        target_name: "fail_on_high_byte".to_string(),
        seed: 42,
        input: FuzzInput {
            data: vec![0x10, 0xFF, 0x20],
            index: 0,
        },
        failure_message: "found high byte".to_string(),
    };

    let report = InputShrinker::shrink(&FailOnHighByteTarget, &replay).unwrap();
    let max_byte = report.shrunk_input.data.iter().max().copied().unwrap_or(0);
    assert!(
        max_byte < 0xFF,
        "shrinker should reduce byte values, got max {max_byte:#x}"
    );
}
