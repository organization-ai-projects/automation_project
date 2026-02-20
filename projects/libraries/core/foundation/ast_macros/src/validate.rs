// projects/libraries/ast_macros/src/validate.rs
#[macro_export]
macro_rules! validate_preset {
    ($node:expr, strict) => {{ ($node).validate_with(&::ast_core::ValidateLimits::strict()) }};
    ($node:expr, unbounded) => {{ ($node).validate_with(&::ast_core::ValidateLimits::unbounded()) }};
    ($node:expr, default) => {{ ($node).validate() }};
    ($node:expr, $unknown:ident) => {{
        ::std::compile_error!(::std::concat!(
            "Unknown validation preset: `",
            ::std::stringify!($unknown),
            "`. Available presets: strict, unbounded, default"
        ))
    }};
}

#[macro_export]
macro_rules! apply_cfg {
    ($limits:ident, ) => {};
    ($limits:ident, max_depth: $d:expr $(, $($rest:tt)*)?) => {{
        $limits.max_depth = $d;
        $( $crate::apply_cfg!($limits, $($rest)*); )?
    }};
    ($limits:ident, max_size: $s:expr $(, $($rest:tt)*)?) => {{
        $limits.max_size = $s;
        $( $crate::apply_cfg!($limits, $($rest)*); )?
    }};
}
