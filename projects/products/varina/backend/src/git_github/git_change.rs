/// Un changement git parsé (porcelain -z).
#[derive(Debug, Clone)]
pub struct GitChange {
    pub xy: [u8; 2], // ex: b'M', b' ' ; b'R', b' '
    pub path: String,
    pub orig_path: Option<String>, // pour renames/copies
}

impl GitChange {
    pub fn status_str(&self) -> String {
        let x = self.xy[0] as char;
        let y = self.xy[1] as char;
        format!("{x}{y}")
    }
}