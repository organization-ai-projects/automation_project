// projects/products/unstable/simulation_compiler/backend/src/generate/pack_emitter.rs
use crate::model::pack_spec::PackSpec;

pub struct PackEmitter;

impl PackEmitter {
    pub fn new() -> Self {
        Self
    }

    /// Emit a deterministic set of (relative-path, content) pairs for the pack scaffold.
    pub fn emit(&self, spec: &PackSpec) -> Vec<(String, Vec<u8>)> {
        let mut artifacts: Vec<(String, Vec<u8>)> = Vec::new();

        // Emit one file per component (1-file=1-type policy).
        let mut components: Vec<_> = spec.components.iter().collect();
        components.sort_by_key(|c| &c.name);
        for comp in components {
            let path = format!("src/components/{}.rs", snake(&comp.name));
            let content = render_component(comp);
            artifacts.push((path, content.into_bytes()));
        }

        // Emit one file per system.
        let mut systems: Vec<_> = spec.systems.iter().collect();
        systems.sort_by_key(|s| &s.name);
        for sys in systems {
            let path = format!("src/systems/{}.rs", snake(&sys.name));
            let content = render_system(sys);
            artifacts.push((path, content.into_bytes()));
        }

        // Emit one file per event.
        let mut events: Vec<_> = spec.events.iter().collect();
        events.sort_by_key(|e| &e.name);
        for ev in events {
            let path = format!("src/events/{}.rs", snake(&ev.name));
            let content = render_event(ev);
            artifacts.push((path, content.into_bytes()));
        }

        // Emit one file per report.
        let mut reports: Vec<_> = spec.reports.iter().collect();
        reports.sort_by_key(|r| &r.name);
        for rep in reports {
            let path = format!("src/reports/{}.rs", snake(&rep.name));
            let content = render_report(rep);
            artifacts.push((path, content.into_bytes()));
        }

        artifacts
    }
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

fn render_component(comp: &crate::model::component_spec::ComponentSpec) -> String {
    let mut s = format!("// Auto-generated component: {}\n", comp.name);
    s.push_str("use serde::{Deserialize, Serialize};\n\n");
    s.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    s.push_str(&format!("pub struct {} {{\n", comp.name));
    for f in &comp.fields {
        s.push_str(&format!("    pub {}: {},\n", f.name, f.ty));
    }
    s.push_str("}\n");
    s
}

fn render_system(sys: &crate::model::system_spec::SystemSpec) -> String {
    let mut s = format!("// Auto-generated system: {}\n", sys.name);
    s.push_str(&format!("pub struct {};\n\n", sys.name));
    s.push_str(&format!("impl {} {{\n", sys.name));
    s.push_str("    pub fn run(&self) {}\n");
    s.push_str("}\n");
    s
}

fn render_event(ev: &crate::model::event_spec::EventSpec) -> String {
    let mut s = format!("// Auto-generated event: {}\n", ev.name);
    s.push_str("use serde::{Deserialize, Serialize};\n\n");
    s.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    s.push_str(&format!("pub struct {} {{\n", ev.name));
    for f in &ev.fields {
        s.push_str(&format!("    pub {}: {},\n", f.name, f.ty));
    }
    s.push_str("}\n");
    s
}

fn render_report(rep: &crate::model::report_spec::ReportSpec) -> String {
    let mut s = format!("// Auto-generated report: {}\n", rep.name);
    s.push_str("use serde::{Deserialize, Serialize};\n\n");
    s.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    s.push_str(&format!("pub struct {} {{\n", rep.name));
    for f in &rep.fields {
        s.push_str(&format!("    pub {}: {},\n", f.name, f.ty));
    }
    s.push_str("}\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parser::Parser;

    #[test]
    fn emit_is_deterministic() {
        let dsl = "component Sensor { value: u32 } component Actor { id: u64 }";
        let mut p = Parser::new(dsl);
        let ast = p.parse().unwrap();
        let spec = PackSpec::from_ast(&ast);
        let emitter = PackEmitter::new();
        let a1 = emitter.emit(&spec);
        let a2 = emitter.emit(&spec);
        assert_eq!(a1, a2);
    }

    #[test]
    fn emit_sorted_alphabetically() {
        let dsl = "component Zebra { z: u8 } component Alpha { a: u8 }";
        let mut p = Parser::new(dsl);
        let ast = p.parse().unwrap();
        let spec = PackSpec::from_ast(&ast);
        let emitter = PackEmitter::new();
        let artifacts = emitter.emit(&spec);
        let paths: Vec<&str> = artifacts.iter().map(|(p, _)| p.as_str()).collect();
        assert!(paths[0] < paths[1], "artifacts must be sorted");
    }
}
