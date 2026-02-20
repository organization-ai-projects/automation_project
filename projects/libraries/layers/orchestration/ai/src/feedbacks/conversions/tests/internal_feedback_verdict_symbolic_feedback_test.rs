use crate::feedbacks::conversions::tests::test_explanation_payload::TestExpectationPayload;
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
            TestExpectationPayload::PositiveWithout,
        ),
        (
            "Rejected verdict",
            InternalFeedbackVerdict::Rejected,
            TestExpectationPayload::NegativeWithout,
        ),
        (
            "NoFeedback verdict",
            InternalFeedbackVerdict::NoFeedback,
            TestExpectationPayload::NegativeWithout,
        ),
        (
            "Incorrect verdict",
            InternalFeedbackVerdict::Incorrect {
                expected_output: "Expected output".into(),
            },
            TestExpectationPayload::NegativeWith("Expected output"),
        ),
        (
            "Partial verdict",
            InternalFeedbackVerdict::Partial {
                correction: "Correction details".into(),
            },
            TestExpectationPayload::NegativeWith("Correction details"),
        ),
    ];

    for (_name, internal_verdict, expectation) in cases {
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        match expectation {
            TestExpectationPayload::PositiveWithout => {
                assert_positive_no_payload(&symbolic_feedback);
            }
            TestExpectationPayload::NegativeWithout => {
                assert_negative_no_payload(&symbolic_feedback);
            }
            TestExpectationPayload::NegativeWith(payload) => {
                assert_negative_with_payload(&symbolic_feedback, payload);
            }
        }
    }
}
