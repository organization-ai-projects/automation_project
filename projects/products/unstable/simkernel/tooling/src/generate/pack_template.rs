#![allow(dead_code)]
pub struct PackTemplate {
    pub pack_name: String,
}
impl PackTemplate {
    pub fn new(pack_name: impl Into<String>) -> Self {
        Self {
            pack_name: pack_name.into(),
        }
    }
    pub fn main_rs(&self) -> String {
        format!(
            "// Pack: {}\npub struct {}Pack;\n",
            self.pack_name, self.pack_name
        )
    }
    pub fn cargo_toml(&self) -> String {
        format!(
            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            self.pack_name.to_lowercase()
        )
    }
}
