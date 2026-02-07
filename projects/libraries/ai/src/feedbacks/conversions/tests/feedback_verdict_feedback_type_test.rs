use crate::feedbacks::conversions::tests::test_helpers::*;
use crate::feedbacks::public_api_feedback::FeedbackVerdict;
use neural::feedback::FeedbackType;

#[test]
fn test_feedback_verdict_to_feedback_type() {
    // Table-driven test cases
    let cases = [
        (
            "Correct verdict",
            FeedbackVerdict::Correct,
            FeedbackTypeExpectation::Correct,
        ),
        (
            "Rejected verdict",
            FeedbackVerdict::Rejected,
            FeedbackTypeExpectation::Incorrect("Rejected"),
        ),
        (
            "NoFeedback verdict",
            FeedbackVerdict::NoFeedback,
            FeedbackTypeExpectation::Incorrect("NoFeedback"),
        ),
        (
            "Incorrect verdict",
            FeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            FeedbackTypeExpectation::Incorrect("Expected output"),
        ),
        (
            "Partial verdict",
            FeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            FeedbackTypeExpectation::Partial("Correction details"),
        ),
    ];

    for (_name, verdict, expectation) in cases {
        let feedback_type: FeedbackType = verdict.into();
        match expectation {
            FeedbackTypeExpectation::Correct => {
                assert_feedback_correct(&feedback_type);
            }
            FeedbackTypeExpectation::Incorrect(output) => {
                assert_feedback_incorrect(&feedback_type, output);
            }
            FeedbackTypeExpectation::Partial(correction) => {
                assert_feedback_partial(&feedback_type, correction);
            }
        }
    }
}

enum FeedbackTypeExpectation {
    Correct,
    Incorrect(&'static str),
    Partial(&'static str),
}
