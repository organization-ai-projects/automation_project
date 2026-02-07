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
            TestExpectation::PositiveWhithout,
        ),
        (
            "Rejected verdict",
            FeedbackVerdict::Rejected,
            TestExpectation::NegativeWithout,
        ),
        (
            "NoFeedback verdict",
            FeedbackVerdict::NoFeedback,
            TestExpectation::NegativeWithout,
        ),
        (
            "Incorrect verdict",
            FeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            TestExpectation::NegativeWith("Expected output"),
        ),
        (
            "Partial verdict",
            FeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            TestExpectation::NegativeWith("Correction details"),
        ),
    ];

    for (_name, verdict, expectation) in cases {
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        match expectation {
            TestExpectation::PositiveWhithout => {
                assert_positive_no_payload(&symbolic_feedback);
            }
            TestExpectation::NegativeWithout => {
                assert_negative_no_payload(&symbolic_feedback);
            }
            TestExpectation::NegativeWith(payload) => {
                assert_negative_with_payload(&symbolic_feedback, payload);
            }
        }
    }
}

enum TestExpectationPayload {
    PositiveWhithout,
    NegativeWithout,
    NegativeWith(&'static str),
}
