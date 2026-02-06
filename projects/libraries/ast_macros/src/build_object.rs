// projects/libraries/ast_macros/src/build_object.rs
#[macro_export]
macro_rules! build_object {
    ({ $($tt:tt)* }) => {{
        let mut fields: ::std::vec::Vec<(::ast_core::AstKey, ::ast_core::AstNode)> = ::std::vec::Vec::new();
        ::ast_core::past!(@obj fields, $($tt)*);
        ::ast_core::AstNode::new(::ast_core::AstKind::Object(fields))
    }};
}
