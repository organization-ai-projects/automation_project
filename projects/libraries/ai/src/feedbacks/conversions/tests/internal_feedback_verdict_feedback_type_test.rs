use crate::feedbacks::conversions::tests::test_helpers::*;
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use neural::feedback::FeedbackType;

#[test]
fn test_internal_feedback_verdict_to_feedback_type() {
    // Table-driven test cases
    let cases = [
        (
            "Correct verdict",
            InternalFeedbackVerdict::Correct,
            FeedbackTypeExpectation::Correct,
        ),
        (
            "Rejected verdict",
            InternalFeedbackVerdict::Rejected,
            FeedbackTypeExpectation::Incorrect("Invalid Feedback"),
        ),
        (
            "NoFeedback verdict",
            InternalFeedbackVerdict::NoFeedback,
            FeedbackTypeExpectation::Incorrect("Invalid Feedback"),
        ),
        (
            "Incorrect verdict",
            InternalFeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            FeedbackTypeExpectation::Incorrect("Expected output"),
        ),
        (
            "Partial verdict",
            InternalFeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            FeedbackTypeExpectation::Partial("Correction details"),
        ),
    ];

    for (_name, internal_verdict, expectation) in cases {
        let feedback_type: FeedbackType = internal_verdict.into();
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
