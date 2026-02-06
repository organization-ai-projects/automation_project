// projects/libraries/symbolic/src/validator/dead_code_visitor.rs
use syn::visit::Visit;
use syn::{Expr, Stmt};

/// Visitor to detect dead code
pub struct DeadCodeVisitor {
    pub dead_code_lines: Vec<usize>,
    current_line: usize,
    found_terminator: bool,
}

impl DeadCodeVisitor {
    pub fn new() -> Self {
        Self {
            dead_code_lines: Vec::new(),
            current_line: 0,
            found_terminator: false,
        }
    }

    /// Check if an expression definitely terminates control flow
    fn expr_terminates(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Return(_) | Expr::Break(_) | Expr::Continue(_) => true,
            Expr::If(expr_if) => {
                // Both branches must terminate for the if expression to terminate
                let then_terminates = self.block_terminates(&expr_if.then_branch);
                let else_terminates = expr_if
                    .else_branch
                    .as_ref()
                    .map(|(_, else_expr)| self.expr_terminates(else_expr))
                    .unwrap_or(false);
                then_terminates && else_terminates
            }
            Expr::Match(expr_match) => {
                // All arms must terminate for the match to terminate
                expr_match
                    .arms
                    .iter()
                    .all(|arm| self.expr_terminates(&arm.body))
            }
            Expr::Block(expr_block) => self.block_terminates(&expr_block.block),
            _ => false,
        }
    }

    /// Check if a block definitely terminates control flow
    fn block_terminates(&self, block: &syn::Block) -> bool {
        let mut found_terminator = false;

        for stmt in &block.stmts {
            let is_terminator = if let Stmt::Expr(expr, _) = stmt {
                self.expr_terminates(expr)
            } else {
                false
            };

            if is_terminator {
                found_terminator = true;
                break;
            }
        }

        found_terminator
    }
}

impl<'ast> Visit<'ast> for DeadCodeVisitor {
    fn visit_block(&mut self, block: &'ast syn::Block) {
        let prev = self.found_terminator;
        self.found_terminator = false;
        syn::visit::visit_block(self, block);
        self.found_terminator = prev;
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.current_line += 1;

        // Check if this statement is a terminator first (before checking if it's dead)
        let is_terminator = if let Stmt::Expr(expr, _) = stmt {
            self.expr_terminates(expr)
        } else {
            false
        };

        // If we already found a terminator, mark this as dead code
        if self.found_terminator && !is_terminator {
            // This statement is dead code
            self.dead_code_lines.push(self.current_line);
        }

        // Set the terminator flag after checking
        if is_terminator {
            self.found_terminator = true;
        }

        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Reset terminator flag when entering a new block, function, or loop
        match expr {
            Expr::Block(_)
            | Expr::Closure(_)
            | Expr::Loop(_)
            | Expr::While(_)
            | Expr::ForLoop(_) => {
                let prev = self.found_terminator;
                self.found_terminator = false;
                syn::visit::visit_expr(self, expr);
                self.found_terminator = prev;
                return;
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}
