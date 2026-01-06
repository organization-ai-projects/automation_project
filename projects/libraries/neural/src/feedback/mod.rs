// Module for feedback loop
pub mod adjust_model;

pub use adjust_model::{FeedbackError, FeedbackType, UserFeedback, FeedbackAdjuster, FeedbackConfig, FeedbackStats, AdjustmentMetrics};