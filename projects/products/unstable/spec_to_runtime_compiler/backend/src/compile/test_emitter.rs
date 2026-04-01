use crate::spec::spec::RuntimeSpec;

pub struct TestEmitter;

impl TestEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, spec: &RuntimeSpec) -> Vec<(String, Vec<u8>)> {
        let mut artifacts = Vec::new();

        let mut transitions: Vec<_> = spec.transitions.iter().collect();
        transitions.sort_by(|a, b| (&a.from, &a.event, &a.to).cmp(&(&b.from, &b.event, &b.to)));

        for t in &transitions {
            let filename = format!(
                "tests/{}_{}_{}",
                t.from.to_lowercase(),
                t.event.to_lowercase(),
                t.to.to_lowercase()
            );
            let golden_path = format!("{}.golden", filename);
            let content = format!(
                "# Golden test: {} -> {} on event '{}'\n\
                 initial_state: {}\n\
                 event: {}\n\
                 expected_state: {}\n",
                t.from, t.to, t.event, t.from, t.event, t.to
            );
            artifacts.push((golden_path, content.into_bytes()));
        }

        artifacts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parser::Parser;
    use crate::spec::spec::RuntimeSpec;

    #[test]
    fn emit_golden_tests_deterministic() {
        let source = "state Idle {}\nstate Running {}\ntransition Idle -> Running on start {}";

        let mut parser1 = Parser::new(source);
        let ast1 = parser1.parse().unwrap();
        let spec1 = RuntimeSpec::from_ast(&ast1);
        let emitter1 = TestEmitter::new();
        let out1 = emitter1.emit(&spec1);

        let mut parser2 = Parser::new(source);
        let ast2 = parser2.parse().unwrap();
        let spec2 = RuntimeSpec::from_ast(&ast2);
        let emitter2 = TestEmitter::new();
        let out2 = emitter2.emit(&spec2);

        assert_eq!(out1.len(), out2.len());
        for (a, b) in out1.iter().zip(out2.iter()) {
            assert_eq!(a.0, b.0);
            assert_eq!(a.1, b.1);
        }
    }
}
