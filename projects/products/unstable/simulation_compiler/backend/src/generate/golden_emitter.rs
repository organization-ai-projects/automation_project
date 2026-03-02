// projects/products/unstable/simulation_compiler/backend/src/generate/golden_emitter.rs
use crate::model::pack_spec::PackSpec;

pub struct GoldenEmitter;

impl GoldenEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, spec: &PackSpec) -> Vec<(String, Vec<u8>)> {
        let mut goldens = Vec::new();
        let mut components: Vec<_> = spec.components.iter().collect();
        components.sort_by_key(|c| &c.name);
        for comp in components {
            let path = format!("goldens/{}.golden", snake_case(&comp.name));
            let content = format!("golden:{}\n", comp.name);
            goldens.push((path, content.into_bytes()));
        }
        goldens
    }
}

fn snake_case(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}
