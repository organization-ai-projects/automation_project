use crate::constraints::constraint::Constraint;
use crate::model::var::Var;

#[derive(Debug, Clone, Default)]
pub struct Ast {
    pub vars: Vec<Var>,
    pub constraints: Vec<Constraint>,
}
