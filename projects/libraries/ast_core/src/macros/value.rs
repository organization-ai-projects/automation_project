// projects/libraries/ast_core/src/macros/value.rs
#[macro_export]
macro_rules! value {
    (null) => { $crate::AstBuilder::null() };
    (true) => { $crate::AstBuilder::bool(true) };
    (false) => { $crate::AstBuilder::bool(false) };
    (- $num:literal) => { $crate::AstBuilder::from(-$num) };
    ([ $($elem:tt),* $(,)? ]) => {{
        $crate::AstBuilder::array(::std::vec![ $( $crate::value!($elem) ),* ])
    }};
    ({ $($tt:tt)* }) => {{
        $crate::build_object!({ $($tt)* })
    }};
    (($e:expr)) => { $crate::AstBuilder::from($e) };
    ($e:expr) => { $crate::AstBuilder::from($e) };
}
