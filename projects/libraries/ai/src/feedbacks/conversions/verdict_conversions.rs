// projects/libraries/ai/src/feedbacks/conversions/verdict_conversions.rs
use neural::feedback::FeedbackType;
use symbolic::feedback_symbolic::SymbolicFeedback;

use crate::feedbacks::{FeedbackVerdict, InternalFeedbackVerdict};

impl<'a> From<FeedbackVerdict<'a>> for SymbolicFeedback {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => SymbolicFeedback::new(true, None),
            FeedbackVerdict::Rejected => SymbolicFeedback::new(false, None),
            FeedbackVerdict::Incorrect { expected_output } => {
                SymbolicFeedback::new(false, Some(expected_output.into_owned()))
            }
            FeedbackVerdict::Partial { correction } => {
                SymbolicFeedback::new(false, Some(correction.into_owned()))
            }
        }
    }
}

impl<'a> From<FeedbackVerdict<'a>> for FeedbackType {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => FeedbackType::Correct {
                metadata: Default::default(),
            },
            FeedbackVerdict::Rejected => FeedbackType::Incorrect {
                expected_output: "Rejected".to_string(),
                metadata: Default::default(),
            },
            FeedbackVerdict::Incorrect { expected_output } => FeedbackType::Incorrect {
                expected_output: expected_output.into_owned(),
                metadata: Default::default(),
            },
            FeedbackVerdict::Partial { correction } => FeedbackType::Partial {
                correction: correction.into_owned(),
                metadata: Default::default(),
            },
        }
    }
}

impl From<InternalFeedbackVerdict> for SymbolicFeedback {
    fn from(verdict: InternalFeedbackVerdict) -> Self {
        match verdict {
            InternalFeedbackVerdict::Correct => SymbolicFeedback::new(true, None),
            InternalFeedbackVerdict::Rejected => SymbolicFeedback::new(false, None),
            InternalFeedbackVerdict::Incorrect { expected_output } => {
                SymbolicFeedback::new(false, Some(expected_output))
            }
            InternalFeedbackVerdict::Partial { correction } => {
                SymbolicFeedback::new(false, Some(correction))
            }
        }
    }
}

impl From<InternalFeedbackVerdict> for FeedbackType {
    fn from(verdict: InternalFeedbackVerdict) -> Self {
        match verdict {
            InternalFeedbackVerdict::Correct => FeedbackType::Correct {
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Rejected => FeedbackType::Incorrect {
                expected_output: "Rejected".to_string(),
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Incorrect { expected_output } => FeedbackType::Incorrect {
                expected_output,
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Partial { correction } => FeedbackType::Partial {
                correction,
                metadata: Default::default(),
            },
        }
    }
}

impl<'a> From<FeedbackVerdict<'a>> for InternalFeedbackVerdict {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => InternalFeedbackVerdict::Correct,
            FeedbackVerdict::Rejected => InternalFeedbackVerdict::Rejected,
            FeedbackVerdict::Incorrect { expected_output } => InternalFeedbackVerdict::Incorrect {
                expected_output: expected_output.into_owned(),
            },
            FeedbackVerdict::Partial { correction } => InternalFeedbackVerdict::Partial {
                correction: correction.into_owned(),
            },
        }
    }
}
