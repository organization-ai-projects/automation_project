use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct PrAuthor {
    #[serde(default)]
    pub(crate) login: Option<String>,
}
