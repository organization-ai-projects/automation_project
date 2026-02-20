// projects/libraries/ast_macros/src/build_array.rs
#[macro_export]
macro_rules! build_array {
    ([ $($elem:tt),* $(,)? ]) => {{
        ::ast_core::AstBuilder::array(::std::vec![ $( ::ast_core::past!(@value $elem) ),* ])
    }};
}
