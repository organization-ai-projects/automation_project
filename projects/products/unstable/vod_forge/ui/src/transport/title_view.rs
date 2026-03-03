use serde::Deserialize;

#[derive(Deserialize)]
pub struct TitleView {
    pub id: String,
    pub name: String,
    pub year: u16,
}
