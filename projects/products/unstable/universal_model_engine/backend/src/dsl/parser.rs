use crate::constraints::constraint::Constraint;
use crate::constraints::constraint_id::ConstraintId;
use crate::diagnostics::backend_error::BackendError;
use crate::dsl::ast::Ast;
use crate::dsl::token::Token;
use crate::model::var::Var;
use crate::model::var_id::VarId;

pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<Ast, BackendError> {
        let mut vars = Vec::new();
        let mut constraints = Vec::new();

        for (index, raw_line) in source.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let tokens: Vec<Token> = line.split_whitespace().map(Token::new).collect();
            if tokens.is_empty() {
                continue;
            }
            let parts: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();

            match parts[0] {
                "var" => {
                    let var = parse_var(&parts).map_err(|m| {
                        BackendError::Validation(format!("line {}: {}", index + 1, m))
                    })?;
                    vars.push(var);
                }
                "constraint" => {
                    let constraint = parse_constraint(&parts).map_err(|m| {
                        BackendError::Validation(format!("line {}: {}", index + 1, m))
                    })?;
                    constraints.push(constraint);
                }
                other => {
                    return Err(BackendError::Validation(format!(
                        "line {}: unknown directive '{}'",
                        index + 1,
                        other
                    )));
                }
            }
        }

        if vars.is_empty() {
            return Err(BackendError::Validation(
                "model must declare at least one variable".to_string(),
            ));
        }

        for constraint in &constraints {
            let exists = vars.iter().any(|v| v.id == constraint.target_var);
            if !exists {
                return Err(BackendError::Validation(format!(
                    "constraint '{}' targets unknown variable '{}'",
                    constraint.id.0, constraint.target_var.0
                )));
            }
        }

        Ok(Ast { vars, constraints })
    }
}

fn parse_var(parts: &[&str]) -> Result<Var, String> {
    if parts.len() < 2 {
        return Err("var requires a name".to_string());
    }
    let name = parts[1];
    let initial_value = match parts.len() {
        2 => 0,
        3 if parts[2] == "int" => 0,
        3 => parts[2]
            .parse::<i64>()
            .map_err(|_| "invalid var value".to_string())?,
        4 if parts[2] == "int" => parts[3]
            .parse::<i64>()
            .map_err(|_| "invalid var value".to_string())?,
        _ => return Err("invalid var syntax".to_string()),
    };
    Ok(Var::integer(name, initial_value))
}

fn parse_constraint(parts: &[&str]) -> Result<Constraint, String> {
    if parts.len() != 5 || parts[3] != "min" {
        return Err("constraint syntax: constraint <id> <var> min <value>".to_string());
    }
    let min_value = parts[4]
        .parse::<i64>()
        .map_err(|_| "invalid constraint min value".to_string())?;

    Ok(Constraint {
        id: ConstraintId(parts[1].to_string()),
        target_var: VarId(parts[2].to_string()),
        min_value,
    })
}
