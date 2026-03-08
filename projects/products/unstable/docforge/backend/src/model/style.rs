use super::style_id::StyleId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Style {
    pub id: StyleId,
    pub font_family: Option<String>,
    pub font_size_pt: Option<u32>,
    pub bold: bool,
    pub italic: bool,
    pub color_hex: Option<String>,
}
