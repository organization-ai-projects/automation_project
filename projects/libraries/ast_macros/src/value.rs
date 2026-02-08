// projects/libraries/ast_macros/src/value.rs
#[macro_export]
macro_rules! value {
    (null) => { ::ast_core::AstBuilder::null() };
    (true) => { ::ast_core::AstBuilder::bool(true) };
    (false) => { ::ast_core::AstBuilder::bool(false) };
    (- $num:literal) => { ::ast_core::AstBuilder::from(-$num) };
    ([ $($elem:tt),* $(,)? ]) => {{
        ::ast_core::AstBuilder::array(::std::vec![ $( $crate::value!($elem) ),* ])
    }};
    ({ $($tt:tt)* }) => {{
        $crate::build_object!({ $($tt)* })
    }};
    (($e:expr)) => { ::ast_core::AstBuilder::from($e) };
    ($e:expr) => { ::ast_core::AstBuilder::from($e) };
}
