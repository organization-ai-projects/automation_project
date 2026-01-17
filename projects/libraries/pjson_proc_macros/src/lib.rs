//! `pjson_proc_macros` - Procedural macros for JSON handling.
//!
//! This crate provides procedural macros for generating JSON structures
//! using the `ast_core` crate for AST representation.
// projects/libraries/pjson_proc_macros/src/lib.rs
extern crate proc_macro;
use ast_core::{AstKind, AstNode};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn pjson(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as syn::Expr);

    // Step 1: Convert the input into an AstNode
    let ast = match convert_to_ast(&parsed_input) {
        Ok(node) => node,
        Err(err) => {
            return syn::Error::new_spanned(parsed_input, err)
                .to_compile_error()
                .into();
        }
    };

    // Step 2: Validate the AST
    if let Err(err) = ast.validate() {
        return syn::Error::new_spanned(parsed_input, err)
            .to_compile_error()
            .into();
    }

    // Step 3: Generate Rust code based on the AST
    let generated_code = generate_code_from_ast(&ast);

    generated_code.into()
}

/// Converts a syn expression into an AstNode
fn convert_to_ast(expr: &syn::Expr) -> Result<AstNode, String> {
    match expr {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(s) => Ok(AstNode::new(AstKind::String(s.value()))),
            syn::Lit::Int(i) => {
                let value = i.base10_parse::<i64>().map_err(|e| e.to_string())?;
                Ok(AstNode::new(AstKind::Number(value.into())))
            }
            syn::Lit::Bool(b) => Ok(AstNode::new(AstKind::Bool(b.value))),
            _ => Err("Unsupported literal type".to_string()),
        },
        syn::Expr::Array(array) => {
            let items = array
                .elems
                .iter()
                .map(convert_to_ast)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(AstKind::Array(items)))
        }
        syn::Expr::Struct(struct_expr) => {
            let fields = struct_expr
                .fields
                .iter()
                .map(|field| {
                    let key = match &field.member {
                        syn::Member::Named(ident) => ident.to_string(),
                        _ => return Err("Field without an identifier".to_string()),
                    };
                    let value = convert_to_ast(&field.expr)?;
                    Ok((key.into(), value))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(AstKind::Object(fields)))
        }
        _ => Err("Unsupported expression".to_string()),
    }
}

/// Generates Rust code from an AstNode
fn generate_code_from_ast(ast: &AstNode) -> proc_macro2::TokenStream {
    match &ast.kind {
        AstKind::String(value) => quote! { #value },
        AstKind::Number(value) => {
            let number_str = format!("{:?}", value);
            quote! { #number_str }
        }
        AstKind::Bool(value) => quote! { #value },
        AstKind::Array(items) => {
            let elements = items.iter().map(generate_code_from_ast);
            quote! { [ #( #elements ),* ] }
        }
        AstKind::Object(fields) => {
            let fields = fields.iter().map(|(key, value)| {
                let key = key.as_str();
                let value = generate_code_from_ast(value);
                quote! { #key: #value }
            });
            quote! { { #( #fields ),* } }
        }
        _ => quote! { null },
    }
}
