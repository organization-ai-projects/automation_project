use crate::feedbacks::conversions::tests::test_helpers::*;
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use symbolic::feedback_symbolic::SymbolicFeedback;

#[test]
fn test_internal_feedback_verdict_to_symbolic_feedback() {
    // Table-driven test cases
    let cases = [
        (
            "Correct verdict",
            InternalFeedbackVerdict::Correct,
            TestExpectation::PositiveNoPayload,
        ),
        (
            "Rejected verdict",
            InternalFeedbackVerdict::Rejected,
            TestExpectation::NegativeNoPayload,
        ),
        (
            "NoFeedback verdict",
            InternalFeedbackVerdict::NoFeedback,
            TestExpectation::NegativeNoPayload,
        ),
        (
            "Incorrect verdict",
            InternalFeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            TestExpectation::NegativeWithPayload("Expected output"),
        ),
        (
            "Partial verdict",
            InternalFeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            TestExpectation::NegativeWithPayload("Correction details"),
        ),
    ];

    for (name, internal_verdict, expectation) in cases {
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        match expectation {
            TestExpectation::PositiveNoPayload => {
                assert_positive_no_payload(&symbolic_feedback);
            }
            TestExpectation::NegativeNoPayload => {
                assert_negative_no_payload(&symbolic_feedback);
            }
            TestExpectation::NegativeWithPayload(payload) => {
                assert_negative_with_payload(&symbolic_feedback, payload);
            }
        }
        // Add context for better error messages
        if symbolic_feedback.is_positive() != expectation.is_positive() {
            panic!("Test case '{}' failed", name);
        }
    }
}

enum TestExpectation {
    PositiveNoPayload,
    NegativeNoPayload,
    NegativeWithPayload(&'static str),
}

impl TestExpectation {
    fn is_positive(&self) -> bool {
        matches!(self, TestExpectation::PositiveNoPayload)
    }
}
