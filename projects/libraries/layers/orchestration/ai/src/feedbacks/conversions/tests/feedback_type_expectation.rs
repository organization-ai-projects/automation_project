pub enum FeedbackTypeExpectation {
    Correct,
    Incorrect(&'static str),
    Partial(&'static str),
}
