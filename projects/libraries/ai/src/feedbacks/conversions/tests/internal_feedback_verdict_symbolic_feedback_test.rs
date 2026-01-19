#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_feedback_verdict_to_symbolic_feedback() {
        // Test case: Correct verdict
        let internal_verdict = InternalFeedbackVerdict::Correct;
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        assert!(symbolic_feedback.is_positive());
        assert!(symbolic_feedback.payload.is_none());

        // Test case: Rejected verdict
        let internal_verdict = InternalFeedbackVerdict::Rejected;
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert!(symbolic_feedback.payload.is_none());

        // Test case: NoFeedback verdict
        let internal_verdict = InternalFeedbackVerdict::NoFeedback;
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert!(symbolic_feedback.payload.is_none());

        // Test case: Incorrect verdict
        let internal_verdict = InternalFeedbackVerdict::Incorrect {
            expected_output: "Expected output".to_string(),
        };
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert_eq!(
            symbolic_feedback.payload,
            Some("Expected output".to_string())
        );

        // Test case: Partial verdict
        let internal_verdict = InternalFeedbackVerdict::Partial {
            correction: "Correction details".to_string(),
        };
        let symbolic_feedback: SymbolicFeedback = internal_verdict.into();
        assert!(!symbolic_feedback.is_positive());
        assert_eq!(
            symbolic_feedback.payload,
            Some("Correction details".to_string())
        );
    }
}
