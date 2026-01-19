#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_feedback_verdict_to_feedback_type() {
        // Test case: Correct verdict
        let internal_verdict = InternalFeedbackVerdict::Correct;
        let feedback_type: FeedbackType = internal_verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Correct {
                metadata: Default::default()
            }
        );

        // Test case: Rejected verdict
        let internal_verdict = InternalFeedbackVerdict::Rejected;
        let feedback_type: FeedbackType = internal_verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Incorrect {
                expected_output: "Rejected".to_string(),
                metadata: Default::default()
            }
        );

        // Test case: NoFeedback verdict
        let internal_verdict = InternalFeedbackVerdict::NoFeedback;
        let feedback_type: FeedbackType = internal_verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Incorrect {
                expected_output: "NoFeedback".to_string(),
                metadata: Default::default()
            }
        );

        // Test case: Incorrect verdict
        let internal_verdict = InternalFeedbackVerdict::Incorrect {
            expected_output: "Expected output".to_string(),
        };
        let feedback_type: FeedbackType = internal_verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Incorrect {
                expected_output: "Expected output".to_string(),
                metadata: Default::default()
            }
        );

        // Test case: Partial verdict
        let internal_verdict = InternalFeedbackVerdict::Partial {
            correction: "Correction details".to_string(),
        };
        let feedback_type: FeedbackType = internal_verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Partial {
                correction: "Correction details".to_string(),
                metadata: Default::default()
            }
        );
    }
}
