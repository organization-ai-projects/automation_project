// projects/libraries/ast_core/src/macros/key.rs
#[macro_export]
macro_rules! key {
    ($k:ident) => {
        $crate::AstKey::Ident(::std::string::String::from(stringify!($k)))
    };
    ($k:literal) => {
        $crate::AstKey::String(::std::string::String::from($k))
    };
    (($k:expr)) => {
        $crate::AstKey::String(::std::string::ToString::to_string(&$k))
    };
}
