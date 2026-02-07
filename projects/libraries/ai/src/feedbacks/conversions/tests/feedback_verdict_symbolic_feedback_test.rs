use crate::feedbacks::conversions::tests::test_helpers::*;
use crate::feedbacks::public_api_feedback::FeedbackVerdict;
use symbolic::feedback_symbolic::SymbolicFeedback;

#[test]
fn test_feedback_verdict_to_symbolic_feedback() {
    // Table-driven test cases
    let cases = [
        (
            "Correct verdict",
            FeedbackVerdict::Correct,
            TestExpectation::PositiveNoPayload,
        ),
        (
            "Rejected verdict",
            FeedbackVerdict::Rejected,
            TestExpectation::NegativeNoPayload,
        ),
        (
            "NoFeedback verdict",
            FeedbackVerdict::NoFeedback,
            TestExpectation::NegativeNoPayload,
        ),
        (
            "Incorrect verdict",
            FeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            TestExpectation::NegativeWithPayload("Expected output"),
        ),
        (
            "Partial verdict",
            FeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            TestExpectation::NegativeWithPayload("Correction details"),
        ),
    ];

    for (_name, verdict, expectation) in cases {
        let symbolic_feedback: SymbolicFeedback = verdict.into();
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
    }
}

enum TestExpectation {
    PositiveNoPayload,
    NegativeNoPayload,
    NegativeWithPayload(&'static str),
}
