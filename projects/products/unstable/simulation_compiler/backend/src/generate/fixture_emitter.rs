// projects/products/unstable/simulation_compiler/backend/src/generate/fixture_emitter.rs
use crate::model::pack_spec::PackSpec;

pub struct FixtureEmitter;

impl FixtureEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, spec: &PackSpec) -> Vec<(String, Vec<u8>)> {
        let mut artifacts = Vec::new();
        let mut components: Vec<_> = spec.components.iter().collect();
        components.sort_by_key(|c| &c.name);
        for comp in components {
            let path = format!("fixtures/{}.json", snake_case(&comp.name));
            let content = render_fixture(&comp.name, &comp.fields);
            artifacts.push((path, content.into_bytes()));
        }
        artifacts
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

fn render_fixture(name: &str, fields: &[crate::model::component_spec::FieldSpec]) -> String {
    let mut s = format!("{{\"type\":\"{name}\"");
    for f in fields {
        s.push_str(&format!(",\"{}\":null", f.name));
    }
    s.push('}');
    s
}
