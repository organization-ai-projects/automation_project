#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_verdict_to_symbolic_feedback() {
        // Test case: Correct verdict
        let verdict = FeedbackVerdict::Correct;
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        assert!(symbolic_feedback.is_positive());
        assert!(symbolic_feedback.payload.is_none());

        // Test case: Rejected verdict
        let verdict = FeedbackVerdict::Rejected;
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert!(symbolic_feedback.payload.is_none());

        // Test case: NoFeedback verdict
        let verdict = FeedbackVerdict::NoFeedback;
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert!(symbolic_feedback.payload.is_none());

        // Test case: Incorrect verdict
        let verdict = FeedbackVerdict::Incorrect {
            expected_output: "Expected output".into(),
        };
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert_eq!(
            symbolic_feedback.payload,
            Some("Expected output".to_string())
        );

        // Test case: Partial verdict
        let verdict = FeedbackVerdict::Partial {
            correction: "Correction details".into(),
        };
        let symbolic_feedback: SymbolicFeedback = verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert_eq!(
            symbolic_feedback.payload,
            Some("Correction details".to_string())
        );
    }
}
