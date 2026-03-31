use crate::diagnostics::error::BackendError;
use crate::dsl::ast::Ast;
use crate::dsl::token::Token;
use crate::model::event_id::EventId;
use crate::model::machine::Transition;
use crate::model::machine_id::MachineId;
use crate::model::state_id::StateId;

pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<Ast, BackendError> {
        let mut ast = Ast::default();
        for (index, raw_line) in source.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let tokens: Vec<Token> = line.split_whitespace().map(Token::new).collect();
            if tokens.is_empty() {
                continue;
            }
            let parts: Vec<&str> = tokens.iter().map(|t| t.lexeme.as_str()).collect();
            Self::parse_line(&mut ast, &parts, index + 1)?;
        }
        Self::validate(&ast)?;
        Ok(ast)
    }

    fn parse_line(ast: &mut Ast, parts: &[&str], line_num: usize) -> Result<(), BackendError> {
        match parts.first().copied() {
            Some("machine") => {
                if parts.len() < 2 {
                    return Err(BackendError::Validation(format!(
                        "line {line_num}: machine requires a name"
                    )));
                }
                ast.machine_id = Some(MachineId(parts[1].to_string()));
                Ok(())
            }
            Some("state") => {
                if parts.len() < 2 {
                    return Err(BackendError::Validation(format!(
                        "line {line_num}: state requires a name"
                    )));
                }
                let sid = StateId(parts[1].to_string());
                if !ast.states.contains(&sid) {
                    ast.states.push(sid);
                }
                Ok(())
            }
            Some("event") => {
                if parts.len() < 2 {
                    return Err(BackendError::Validation(format!(
                        "line {line_num}: event requires a name"
                    )));
                }
                let eid = EventId(parts[1].to_string());
                if !ast.events.contains(&eid) {
                    ast.events.push(eid);
                }
                Ok(())
            }
            Some("initial") => {
                if parts.len() < 2 {
                    return Err(BackendError::Validation(format!(
                        "line {line_num}: initial requires a state name"
                    )));
                }
                ast.initial_state = Some(StateId(parts[1].to_string()));
                Ok(())
            }
            Some("var") => {
                if parts.len() < 3 {
                    return Err(BackendError::Validation(format!(
                        "line {line_num}: var requires name and value"
                    )));
                }
                let val: i64 = parts[2].parse().map_err(|_| {
                    BackendError::Validation(format!(
                        "line {line_num}: invalid variable value '{}'",
                        parts[2]
                    ))
                })?;
                ast.variables.insert(parts[1].to_string(), val);
                Ok(())
            }
            Some("transition") => {
                // transition <from_state> <event> -> <to_state> [guard:<expr>] [action:<expr>]
                if parts.len() < 5 || parts[3] != "->" {
                    return Err(BackendError::Validation(format!(
                        "line {line_num}: transition syntax: transition <from> <event> -> <to> [guard:<expr>] [action:<expr>]"
                    )));
                }
                let from = StateId(parts[1].to_string());
                let event = EventId(parts[2].to_string());
                let target = StateId(parts[4].to_string());
                let mut guard = None;
                let mut action = None;
                for part in &parts[5..] {
                    if let Some(g) = part.strip_prefix("guard:") {
                        guard = Some(g.to_string());
                    } else if let Some(a) = part.strip_prefix("action:") {
                        action = Some(a.to_string());
                    }
                }
                ast.transitions.push((
                    from,
                    Transition {
                        event,
                        target,
                        guard,
                        action,
                    },
                ));
                Ok(())
            }
            Some(other) => Err(BackendError::Validation(format!(
                "line {line_num}: unknown directive '{other}'"
            ))),
            None => Ok(()),
        }
    }

    fn validate(ast: &Ast) -> Result<(), BackendError> {
        if ast.machine_id.is_none() {
            return Err(BackendError::Validation(
                "machine must have a name (use 'machine <name>')".to_string(),
            ));
        }
        if ast.states.is_empty() {
            return Err(BackendError::Validation(
                "machine must have at least one state".to_string(),
            ));
        }
        if ast.initial_state.is_none() {
            return Err(BackendError::Validation(
                "machine must have an initial state (use 'initial <state>')".to_string(),
            ));
        }
        let initial = ast.initial_state.as_ref().unwrap();
        if !ast.states.contains(initial) {
            return Err(BackendError::Validation(format!(
                "initial state '{}' is not a declared state",
                initial.0
            )));
        }
        for (from, t) in &ast.transitions {
            if !ast.states.contains(from) {
                return Err(BackendError::Validation(format!(
                    "transition source '{}' is not a declared state",
                    from.0
                )));
            }
            if !ast.states.contains(&t.target) {
                return Err(BackendError::Validation(format!(
                    "transition target '{}' is not a declared state",
                    t.target.0
                )));
            }
            if !ast.events.contains(&t.event) {
                return Err(BackendError::Validation(format!(
                    "transition event '{}' is not a declared event",
                    t.event.0
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_machine() {
        let src = "\
machine toggle
state off
state on
event flip
initial off
transition off flip -> on
transition on flip -> off
";
        let ast = Parser::parse(src).unwrap();
        assert_eq!(ast.machine_id.unwrap().0, "toggle");
        assert_eq!(ast.states.len(), 2);
        assert_eq!(ast.events.len(), 1);
        assert_eq!(ast.transitions.len(), 2);
    }

    #[test]
    fn parse_rejects_missing_machine() {
        let src = "state a\ninitial a\n";
        assert!(Parser::parse(src).is_err());
    }

    #[test]
    fn parse_rejects_unknown_directive() {
        let src = "machine m\nstate a\ninitial a\nfoo bar\n";
        assert!(Parser::parse(src).is_err());
    }

    #[test]
    fn parse_with_variables_and_guards() {
        let src = "\
machine counter
state idle
state counting
event inc
initial idle
var count 0
transition idle inc -> counting action:count+=1
transition counting inc -> counting guard:count<10 action:count+=1
";
        let ast = Parser::parse(src).unwrap();
        assert_eq!(ast.variables.get("count"), Some(&0));
        assert_eq!(ast.transitions.len(), 2);
        assert!(ast.transitions[1].1.guard.is_some());
    }
}
