#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    CodeSpan(String),
    Link { href: String, text: String },
}
