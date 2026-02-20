// projects/libraries/ast_core/src/ast_macro.rs
// High-level AST macro for building and validating nodes.
#[macro_export]
macro_rules! past {
    // ============================================================
    // OBJECT WITH TAIL (meta/validate) - MUST BE FIRST to avoid
    // being captured by the simpler object-only rule
    // ============================================================

    // BUILD + META (IA-friendly) then (optional) validate
    ({ $($tt:tt)* }, $($tail:tt)+ ) => {{
        let mut node = $crate::past!(@build_object { $($tt)* });
        $crate::past!(@apply_tail node, $($tail)+)
    }};

    // ============================================================
    // SIMPLE BUILD MODES (no tail)
    // ============================================================

    // BUILD OBJECT only
    ({ $($tt:tt)* }) => {{
        $crate::past!(@build_object { $($tt)* })
    }};

    // BUILD ARRAY (top-level)
    ([ $($elem:tt),* $(,)? ]) => {{
        $crate::past!(@build_array [ $($elem),* ])
    }};

    // BUILD SCALARS (top-level)
    (null) => {{ $crate::AstBuilder::null() }};
    (true) => {{ $crate::AstBuilder::bool(true) }};
    (false) => {{ $crate::AstBuilder::bool(false) }};

    // Negative number literal
    (- $num:literal) => {{ $crate::AstBuilder::from(-$num) }};

    // ============================================================
    // VALIDATE EXISTING NODE
    // ============================================================
    ($node:expr, validate) => {{
        ($node).validate()
    }};
    ($node:expr, validate: preset: $preset:ident) => {{
        $crate::past!(@validate_preset $node, $preset)
    }};
    ($node:expr, validate: cfg: { $($cfg:tt)* }) => {{
        let mut limits = $crate::ValidateLimits::default();
        $crate::past!(@apply_cfg limits, $($cfg)*);
        ($node).validate_with(&limits)
    }};
    ($node:expr, validate: with: $limits:expr) => {{
        ($node).validate_with(&$limits)
    }};

    // ============================================================
    // FALLBACK: Any other expression (positive literals, variables)
    // MUST BE LAST to not capture other patterns
    // ============================================================
    ($e:expr) => {{ $crate::AstBuilder::from($e) }};

    // ============================================================
    // INTERNALS
    // ============================================================

    // --- build object from DSL ---
    (@build_object { $($tt:tt)* }) => {{
        let mut fields: ::std::vec::Vec<($crate::AstKey, $crate::AstNode)> = ::std::vec::Vec::new();
        $crate::past!(@obj fields, $($tt)*);
        $crate::AstBuilder::object(fields)
    }};

    // --- build array ---
    (@build_array [ $($elem:tt),* $(,)? ]) => {{
        $crate::AstBuilder::array(::std::vec![ $( $crate::past!(@value $elem) ),* ])
    }};

    // --- object muncher end ---
    (@obj $fields:ident,) => {};
    (@obj $fields:ident) => {};

    // key: value, rest...
    (@obj $fields:ident, $key:tt : $($rest:tt)+) => {{
        let k: $crate::AstKey = $crate::past!(@key $key);
        $crate::past!(@val_push $fields, k, (), $($rest)+);
    }};

    // accumulate value tokens until comma
    (@val_push $fields:ident, $k:expr, ($($val:tt)*), , $($tail:tt)*) => {{
        let v: $crate::AstNode = $crate::past!(@value $($val)*);
        $fields.push(($k, v));
        $crate::past!(@obj $fields, $($tail)*);
    }};
    // last pair without trailing comma
    (@val_push $fields:ident, $k:expr, ($($val:tt)*),) => {{
        let v: $crate::AstNode = $crate::past!(@value $($val)*);
        $fields.push(($k, v));
    }};
    // munch one token
    (@val_push $fields:ident, $k:expr, ($($val:tt)*), $head:tt $($tail:tt)*) => {{
        $crate::past!(@val_push $fields, $k, ($($val)* $head), $($tail)*);
    }};

    // --- keys: ident | "string" | (expr -> ToString) ---
    (@key $k:ident) => { $crate::AstKey::Ident(::std::string::String::from(stringify!($k))) };
    (@key $k:literal) => { $crate::AstKey::String(::std::string::String::from($k)) };
    (@key ($k:expr)) => { $crate::AstKey::String(::std::string::ToString::to_string(&$k)) };
    (@key $k:expr) => { $crate::AstKey::from(&$k) };

    // --- values: null/true/false, arrays, nested objects, or expression ---
    (@value null) => { $crate::AstBuilder::null() };
    (@value true) => { $crate::AstBuilder::bool(true) };
    (@value false) => { $crate::AstBuilder::bool(false) };

    // negative number literal (must come before fallback expr)
    (@value - $num:literal) => { $crate::AstBuilder::from(-$num) };

    // array: [a, b, c]
    (@value [ $($elem:tt),* $(,)? ]) => {{
        $crate::AstBuilder::array(::std::vec![ $( $crate::past!(@value $elem) ),* ])
    }};

    // nested object: { ... }
    (@value { $($tt:tt)* }) => {{
        $crate::past!(@build_object { $($tt)* })
    }};

    // parenthesized expr (helps turbofish etc.)
    (@value ($e:expr)) => { $crate::AstBuilder::from($e) };

    // fallback: treat as expression
    (@value $e:expr) => { $crate::AstBuilder::from($e) };

    // --- validate presets ---
    (@validate_preset $node:expr, strict) => {{
        ($node).validate_with(&$crate::ValidateLimits::strict())
    }};
    (@validate_preset $node:expr, unbounded) => {{
        ($node).validate_with(&$crate::ValidateLimits::unbounded())
    }};
    (@validate_preset $node:expr, default) => {{
        ($node).validate()
    }};
    // Catch-all for unknown presets - produces clear compile error
    (@validate_preset $node:expr, $unknown:ident) => {{
        ::std::compile_error!(::std::concat!(
            "Unknown validation preset: `",
            ::std::stringify!($unknown),
            "`. Available presets: strict, unbounded, default"
        ))
    }};

    // --- apply cfg for ValidateLimits ---
    (@apply_cfg $limits:ident, ) => {};
    (@apply_cfg $limits:ident, max_depth: $d:expr $(, $($rest:tt)*)?) => {{
        $limits.max_depth = $d;
        $( $crate::past!(@apply_cfg $limits, $($rest)*); )?
    }};
    (@apply_cfg $limits:ident, max_size: $s:expr $(, $($rest:tt)*)?) => {{
        $limits.max_size = $s;
        $( $crate::past!(@apply_cfg $limits, $($rest)*); )?
    }};

    // --- tail: allow chaining meta + validate in any order ---
    // Base case: no more tail, return node
    (@apply_tail $node:expr, $($tail:tt)*) => {{
        let mut node = $node;
        $crate::past!(@apply_tail_inner node, $($tail)*)
    }};

    (@apply_tail_inner $node:ident, ) => { $node };
    (@apply_tail_inner $node:ident, origin: ai($name:expr) $(, $($rest:tt)*)? ) => {{
        $node.meta.origin = ::std::option::Option::Some($crate::Origin::Ai($name));
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, origin: tool($name:expr) $(, $($rest:tt)*)? ) => {{
        $node.meta.origin = ::std::option::Option::Some($crate::Origin::Tool($name));
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, origin: parser($name:expr) $(, $($rest:tt)*)? ) => {{
        $node.meta.origin = ::std::option::Option::Some($crate::Origin::Parser($name));
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, origin: proc($name:expr) $(, $($rest:tt)*)? ) => {{
        $node.meta.origin = ::std::option::Option::Some($crate::Origin::ProcMacro($name));
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, flags: [ $($flag:literal),* $(,)? ] $(, $($rest:tt)*)? ) => {{
        $( $node.meta.flags.insert($flag); )*
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, attrs: { $($k:literal : $v:expr),* $(,)? } $(, $($rest:tt)*)? ) => {{
        $( $node.meta.attrs.insert($k, ::std::string::ToString::to_string(&$v)); )*
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, validate $(, $($rest:tt)*)? ) => {{
        $node.validate()?;
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, validate: preset: $preset:ident $(, $($rest:tt)*)? ) => {{
        $crate::past!(@validate_preset $node, $preset)?;
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, validate: cfg: { $($cfg:tt)* } $(, $($rest:tt)*)? ) => {{
        let mut limits = $crate::ValidateLimits::default();
        $crate::past!(@apply_cfg limits, $($cfg)*);
        $node.validate_with(&limits)?;
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
    (@apply_tail_inner $node:ident, validate: with: $limits:expr $(, $($rest:tt)*)? ) => {{
        $node.validate_with(&$limits)?;
        $crate::past!(@apply_tail_inner $node, $($($rest)*)?)
    }};
}
