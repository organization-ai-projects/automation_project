use crate::spec::spec::RuntimeSpec;

pub struct RuntimeEmitter;

impl RuntimeEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, spec: &RuntimeSpec) -> Vec<(String, Vec<u8>)> {
        let mut artifacts = Vec::new();
        artifacts.push(("src/state.rs".to_string(), self.emit_state(spec)));
        artifacts.push(("src/transition.rs".to_string(), self.emit_transition(spec)));
        artifacts.push(("src/runner.rs".to_string(), self.emit_runner(spec)));
        artifacts.push(("src/main.rs".to_string(), self.emit_main()));
        artifacts
    }

    fn emit_state(&self, spec: &RuntimeSpec) -> Vec<u8> {
        let mut out = String::new();
        out.push_str("// Generated state definitions\n\n");

        out.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        out.push_str("pub enum State {\n");
        for state in &spec.states {
            if state.fields.is_empty() {
                out.push_str(&format!("    {},\n", state.name));
            } else {
                out.push_str(&format!("    {} {{\n", state.name));
                for field in &state.fields {
                    out.push_str(&format!("        {}: {},\n", field.name, field.ty));
                }
                out.push_str("    },\n");
            }
        }
        out.push_str("}\n");

        out.into_bytes()
    }

    fn emit_transition(&self, spec: &RuntimeSpec) -> Vec<u8> {
        let mut out = String::new();
        out.push_str("// Generated transition function\n\n");
        out.push_str("use crate::state::State;\n\n");

        out.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        out.push_str("pub enum Event {\n");
        let mut events: Vec<&str> = spec.transitions.iter().map(|t| t.event.as_str()).collect();
        events.sort();
        events.dedup();
        for event in &events {
            out.push_str(&format!("    {},\n", capitalize(event)));
        }
        out.push_str("}\n\n");

        out.push_str(
            "pub fn apply_transition(state: &State, event: &Event, _tick: u64) -> Option<State> {\n",
        );
        out.push_str("    match (state, event) {\n");
        for t in &spec.transitions {
            let from_pattern = if spec
                .states
                .iter()
                .any(|s| s.name == t.from && !s.fields.is_empty())
            {
                format!("State::{} {{ .. }}", t.from)
            } else {
                format!("State::{}", t.from)
            };
            let to_expr = if spec
                .states
                .iter()
                .any(|s| s.name == t.to && !s.fields.is_empty())
            {
                let defaults: Vec<String> = spec
                    .states
                    .iter()
                    .find(|s| s.name == t.to)
                    .unwrap()
                    .fields
                    .iter()
                    .map(|f| format!("{}: Default::default() /* TODO: replace with actual value */", f.name))
                    .collect();
                format!("State::{} {{ {} }}", t.to, defaults.join(", "))
            } else {
                format!("State::{}", t.to)
            };
            out.push_str(&format!(
                "        ({}, Event::{}) => Some({}),\n",
                from_pattern,
                capitalize(&t.event),
                to_expr
            ));
        }
        out.push_str("        _ => None,\n");
        out.push_str("    }\n");
        out.push_str("}\n");

        out.into_bytes()
    }

    fn emit_runner(&self, spec: &RuntimeSpec) -> Vec<u8> {
        let mut out = String::new();
        out.push_str("// Generated deterministic runner (logical ticks only)\n\n");
        out.push_str("use crate::state::State;\n");
        out.push_str("use crate::transition::{Event, apply_transition};\n\n");

        out.push_str("pub struct Runner {\n");
        out.push_str("    pub state: State,\n");
        out.push_str("    pub tick: u64,\n");
        out.push_str("}\n\n");

        // Use explicit initial state from spec, or fall back to first declared state.
        let initial = spec
            .initial_state
            .as_deref()
            .and_then(|name| spec.states.iter().find(|s| s.name == name))
            .or_else(|| spec.states.first());

        let initial_state = if let Some(first) = initial {
            if first.fields.is_empty() {
                format!("State::{}", first.name)
            } else {
                let defaults: Vec<String> = first
                    .fields
                    .iter()
                    .map(|f| format!("{}: Default::default() /* TODO: replace with actual value */", f.name))
                    .collect();
                format!("State::{} {{ {} }}", first.name, defaults.join(", "))
            }
        } else {
            return b"// No states defined\n".to_vec();
        };

        out.push_str("impl Runner {\n");
        out.push_str(&format!(
            "    pub fn new() -> Self {{\n        Self {{ state: {}, tick: 0 }}\n    }}\n\n",
            initial_state
        ));
        out.push_str("    pub fn process_event(&mut self, event: &Event) -> bool {\n");
        out.push_str(
            "        if let Some(next) = apply_transition(&self.state, event, self.tick) {\n",
        );
        out.push_str("            self.state = next;\n");
        out.push_str("            self.tick += 1;\n");
        out.push_str("            true\n");
        out.push_str("        } else {\n");
        out.push_str("            false\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");

        out.into_bytes()
    }

    fn emit_main(&self) -> Vec<u8> {
        let mut out = String::new();
        out.push_str("// Generated runner entry point\n\n");
        out.push_str("mod state;\n");
        out.push_str("mod transition;\n");
        out.push_str("mod runner;\n\n");
        out.push_str("fn main() {\n");
        out.push_str("    let runner = runner::Runner::new();\n");
        out.push_str("    println!(\"Runner initialized at tick {}\", runner.tick);\n");
        out.push_str("    println!(\"Initial state: {:?}\", runner.state);\n");
        out.push_str("}\n");
        out.into_bytes()
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parser::Parser;
    use crate::spec::spec::RuntimeSpec;

    #[test]
    fn emit_is_deterministic() {
        let source =
            "state Idle {}\nstate Running { tick: u64 }\ntransition Idle -> Running on start {}";

        let mut parser1 = Parser::new(source);
        let ast1 = parser1.parse().unwrap();
        let spec1 = RuntimeSpec::from_ast(&ast1);
        let emitter1 = RuntimeEmitter::new();
        let out1 = emitter1.emit(&spec1);

        let mut parser2 = Parser::new(source);
        let ast2 = parser2.parse().unwrap();
        let spec2 = RuntimeSpec::from_ast(&ast2);
        let emitter2 = RuntimeEmitter::new();
        let out2 = emitter2.emit(&spec2);

        assert_eq!(out1.len(), out2.len());
        for (a, b) in out1.iter().zip(out2.iter()) {
            assert_eq!(a.0, b.0);
            assert_eq!(a.1, b.1);
        }
    }
}
