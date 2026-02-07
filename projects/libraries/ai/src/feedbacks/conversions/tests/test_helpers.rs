// projects/libraries/ai/src/feedbacks/conversions/tests/test_helpers.rs
use neural::feedback::FeedbackType;
use symbolic::feedback_symbolic::SymbolicFeedback;

/// Helper function to assert that a SymbolicFeedback is positive with no metadata
pub(crate) fn assert_positive_no_payload(feedback: &SymbolicFeedback) {
    assert!(feedback.is_positive());
    assert!(feedback.metadata.is_none());
}

/// Helper function to assert that a SymbolicFeedback is negative with no metadata
pub(crate) fn assert_negative_no_payload(feedback: &SymbolicFeedback) {
    assert!(!feedback.is_positive());
    assert!(feedback.metadata.is_none());
}

/// Helper function to assert that a SymbolicFeedback is negative with a specific metadata payload
pub(crate) fn assert_negative_with_payload(feedback: &SymbolicFeedback, expected_metadata: &str) {
    assert!(!feedback.is_positive());
    assert_eq!(feedback.metadata, Some(expected_metadata.to_string()));
}

/// Helper function to assert that a FeedbackType is Correct
pub(crate) fn assert_feedback_correct(feedback: &FeedbackType) {
    assert!(
        matches!(feedback, FeedbackType::Correct { .. }),
        "Expected FeedbackType::Correct, got {:?}",
        feedback
    );
}

/// Helper function to assert that a FeedbackType is Incorrect with a specific output
pub(crate) fn assert_feedback_incorrect(feedback: &FeedbackType, expected_output: &str) {
    match feedback {
        FeedbackType::Incorrect {
            expected_output: actual,
            ..
        } => {
            assert_eq!(actual, expected_output);
        }
        _ => panic!("Expected FeedbackType::Incorrect, got {:?}", feedback),
    }
}

/// Helper function to assert that a FeedbackType is Partial with a specific correction
pub(crate) fn assert_feedback_partial(feedback: &FeedbackType, correction: &str) {
    match feedback {
        FeedbackType::Partial {
            correction: actual, ..
        } => {
            assert_eq!(actual, correction);
        }
        _ => panic!("Expected FeedbackType::Partial, got {:?}", feedback),
    }
}
