// projects/libraries/ast_core/src/macros/build_array.rs
#[macro_export]
macro_rules! build_array {
    ([ $($elem:tt),* $(,)? ]) => {{
        $crate::AstBuilder::array(::std::vec![ $( $crate::past!(@value $elem) ),* ])
    }};
}
