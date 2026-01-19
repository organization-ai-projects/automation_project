#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_verdict_to_feedback_type() {
        // Test case: Correct verdict
        let verdict = FeedbackVerdict::Correct;
        let feedback_type: FeedbackType = verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Correct {
                metadata: Default::default()
            }
        );

        // Test case: Rejected verdict
        let verdict = FeedbackVerdict::Rejected;
        let feedback_type: FeedbackType = verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Incorrect {
                expected_output: "Rejected".to_string(),
                metadata: Default::default()
            }
        );

        // Test case: NoFeedback verdict
        let verdict = FeedbackVerdict::NoFeedback;
        let feedback_type: FeedbackType = verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Incorrect {
                expected_output: "NoFeedback".to_string(),
                metadata: Default::default()
            }
        );

        // Test case: Incorrect verdict
        let verdict = FeedbackVerdict::Incorrect {
            expected_output: "Expected output".into(),
        };
        let feedback_type: FeedbackType = verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Incorrect {
                expected_output: "Expected output".to_string(),
                metadata: Default::default()
            }
        );

        // Test case: Partial verdict
        let verdict = FeedbackVerdict::Partial {
            correction: "Correction details".into(),
        };
        let feedback_type: FeedbackType = verdict.into();
        assert_eq!(
            feedback_type,
            FeedbackType::Partial {
                correction: "Correction details".to_string(),
                metadata: Default::default()
            }
        );
    }
}
