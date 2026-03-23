#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrFieldName {
    State,
    BaseRefName,
    HeadRefName,
    Title,
    Body,
    AuthorLogin,
    CommitMessages,
}
