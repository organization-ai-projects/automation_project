// projects/libraries/ast_core/src/macros/build_object.rs
#[macro_export]
macro_rules! build_object {
    ({ $($tt:tt)* }) => {{
        let mut fields: ::std::vec::Vec<($crate::AstKey, $crate::AstNode)> = ::std::vec::Vec::new();
        $crate::past!(@obj fields, $($tt)*);
        $crate::AstNode::new($crate::AstKind::Object(fields))
    }};
}
