use crate::feedbacks::conversions::tests::test_explanation_payload::TestExpectationPayload;
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
            TestExpectationPayload::PositiveWithout,
        ),
        (
            "Rejected verdict",
            FeedbackVerdict::Rejected,
            TestExpectationPayload::NegativeWithout,
        ),
        (
            "NoFeedback verdict",
            FeedbackVerdict::NoFeedback,
            TestExpectationPayload::NegativeWithout,
        ),
        (
            "Incorrect verdict",
            FeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            TestExpectationPayload::NegativeWith("Expected output"),
        ),
        (
            "Partial verdict",
            FeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            TestExpectationPayload::NegativeWith("Correction details"),
        ),
    ];

    for (name, verdict, expectation) in cases {
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        let result = std::panic::catch_unwind(|| match expectation {
            TestExpectationPayload::PositiveWithout => {
                assert_positive_no_payload(&symbolic_feedback);
            }
            TestExpectationPayload::NegativeWithout => {
                assert_negative_no_payload(&symbolic_feedback);
            }
            TestExpectationPayload::NegativeWith(payload) => {
                assert_negative_with_payload(&symbolic_feedback, payload);
            }
        });

        if result.is_err() {
            panic!("table test case '{name}' failed");
        }
    }
}
