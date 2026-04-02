//! projects/products/unstable/rust_language/backend/src/compiler/transpiler.rs
use crate::diagnostics::Error;
use crate::model::RhlAst;

pub(crate) struct Transpiler;

impl Transpiler {
    pub(crate) fn transpile(ast: &RhlAst) -> Result<String, Error> {
        let mut output = String::new();
        Self::emit(ast, &mut output, 0)?;
        Ok(output)
    }

    fn emit(node: &RhlAst, out: &mut String, indent: usize) -> Result<(), Error> {
        match node {
            RhlAst::Program(items) => {
                for item in items {
                    Self::emit(item, out, indent)?;
                    out.push('\n');
                }
            }
            RhlAst::FnDecl {
                name,
                params,
                return_type,
                body,
            } => {
                Self::write_indent(out, indent);
                out.push_str("fn ");
                out.push_str(name);
                out.push('(');
                for (i, (pname, ptype)) in params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(pname);
                    out.push_str(": ");
                    out.push_str(ptype);
                }
                out.push(')');
                if let Some(ret) = return_type {
                    out.push_str(" -> ");
                    out.push_str(ret);
                }
                out.push_str(" {\n");
                for stmt in body {
                    Self::emit(stmt, out, indent + 1)?;
                    out.push('\n');
                }
                Self::write_indent(out, indent);
                out.push('}');
            }
            RhlAst::StructDecl { name, fields } => {
                Self::write_indent(out, indent);
                out.push_str("struct ");
                out.push_str(name);
                out.push_str(" {\n");
                for (fname, ftype) in fields {
                    Self::write_indent(out, indent + 1);
                    out.push_str(fname);
                    out.push_str(": ");
                    out.push_str(ftype);
                    out.push_str(",\n");
                }
                Self::write_indent(out, indent);
                out.push('}');
            }
            RhlAst::LetBinding {
                name,
                mutable,
                type_annotation,
                value,
            } => {
                Self::write_indent(out, indent);
                out.push_str("let ");
                if *mutable {
                    out.push_str("mut ");
                }
                out.push_str(name);
                if let Some(ty) = type_annotation {
                    out.push_str(": ");
                    out.push_str(ty);
                }
                out.push_str(" = ");
                Self::emit(value, out, 0)?;
                out.push(';');
            }
            RhlAst::Return(expr) => {
                Self::write_indent(out, indent);
                out.push_str("return ");
                Self::emit(expr, out, 0)?;
                out.push(';');
            }
            RhlAst::IfExpr {
                condition,
                then_body,
                else_body,
            } => {
                Self::write_indent(out, indent);
                out.push_str("if ");
                Self::emit(condition, out, 0)?;
                out.push_str(" {\n");
                for stmt in then_body {
                    Self::emit(stmt, out, indent + 1)?;
                    out.push('\n');
                }
                Self::write_indent(out, indent);
                out.push('}');
                if let Some(else_stmts) = else_body {
                    out.push_str(" else {\n");
                    for stmt in else_stmts {
                        Self::emit(stmt, out, indent + 1)?;
                        out.push('\n');
                    }
                    Self::write_indent(out, indent);
                    out.push('}');
                }
            }
            RhlAst::WhileLoop { condition, body } => {
                Self::write_indent(out, indent);
                out.push_str("while ");
                Self::emit(condition, out, 0)?;
                out.push_str(" {\n");
                for stmt in body {
                    Self::emit(stmt, out, indent + 1)?;
                    out.push('\n');
                }
                Self::write_indent(out, indent);
                out.push('}');
            }
            RhlAst::BinaryOp { left, op, right } => {
                Self::emit(left, out, 0)?;
                out.push(' ');
                out.push_str(op);
                out.push(' ');
                Self::emit(right, out, 0)?;
            }
            RhlAst::Call { callee, args } => {
                out.push_str(callee);
                out.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    Self::emit(arg, out, 0)?;
                }
                out.push(')');
            }
            RhlAst::FieldAccess { object, field } => {
                Self::emit(object, out, 0)?;
                out.push('.');
                out.push_str(field);
            }
            RhlAst::Identifier(name) => {
                out.push_str(name);
            }
            RhlAst::IntLiteral(v) => {
                out.push_str(&v.to_string());
            }
            RhlAst::FloatLiteral(v) => {
                let s = format!("{v}");
                out.push_str(&s);
                if !s.contains('.') {
                    out.push_str(".0");
                }
            }
            RhlAst::StringLiteral(s) => {
                out.push('"');
                let escaped: String = s.chars().flat_map(|c| c.escape_default()).collect();
                out.push_str(&escaped);
                out.push('"');
            }
            RhlAst::BoolLiteral(b) => {
                out.push_str(if *b { "true" } else { "false" });
            }
            RhlAst::Block(stmts) => {
                out.push_str("{\n");
                for stmt in stmts {
                    Self::emit(stmt, out, indent + 1)?;
                    out.push('\n');
                }
                Self::write_indent(out, indent);
                out.push('}');
            }
            RhlAst::Assignment { target, value } => {
                Self::write_indent(out, indent);
                out.push_str(target);
                out.push_str(" = ");
                Self::emit(value, out, 0)?;
                out.push(';');
            }
        }
        Ok(())
    }

    fn write_indent(out: &mut String, level: usize) {
        for _ in 0..level {
            out.push_str("    ");
        }
    }
}
