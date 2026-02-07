// projects/libraries/ast_macros/src/key.rs
#[macro_export]
macro_rules! key {
    ($k:ident) => {
        ::ast_core::AstKey::Ident(::std::string::String::from(stringify!($k)))
    };
    ($k:literal) => {
        ::ast_core::AstKey::String(::std::string::String::from($k))
    };
    (($k:expr)) => {
        ::ast_core::AstKey::String(::std::string::ToString::to_string(&$k))
    };
}
