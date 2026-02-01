//! JSON construction macros optimized for performance.
//!
// projects/libraries/common_json/src/macros.rs
#[macro_export]
macro_rules! pjson {
    // NULL
    (null) => {
        $crate::Json::Null
    };

    // BOOLEANS
    (true) => {
        $crate::Json::Bool(true)
    };
    (false) => {
        $crate::Json::Bool(false)
    };

    // ARRAYS
    ([]) => {
        $crate::Json::Array(::std::vec::Vec::new())
    };
    ([ $($elem:tt),* $(,)? ]) => {
        $crate::Json::Array(::std::vec![ $($crate::pjson!($elem)),* ])
    };

    // OBJECTS
    ({}) => {
        $crate::Json::Object($crate::JsonMap::new())
    };
    ({ $($tt:tt)* }) => {{
        let mut map = $crate::JsonMap::new();
        $crate::pjson_object!(@parse map, $($tt)*);
        $crate::Json::Object(map)
    }};

    // NEGATIVE NUMBERS
    (- $num:literal) => {
        $crate::Json::from(-$num)
    };
    (- $expr:expr) => {
        $crate::Json::from(-$expr)
    };

    // STRING LITERALS
    ($s:literal) => {
        $crate::Json::from($s)
    };

    // EXPRESSIONS
    ($other:expr) => {
        $crate::Json::from($other)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pjson_val {
    // Special keywords
    (null) => { $crate::Json::Null };
    (true) => { $crate::Json::Bool(true) };
    (false) => { $crate::Json::Bool(false) };

    // Nested array
    ([ $($elem:tt),* $(,)? ]) => {
        $crate::Json::Array(::std::vec![ $($crate::pjson!($elem)),* ])
    };

    // Nested object
    ({ $($tt:tt)* }) => {
        $crate::pjson!({ $($tt)* })
    };

    // Expression in parentheses (for method calls, complex expressions)
    (($expr:expr)) => {
        $crate::Json::from($expr)
    };

    // Negative number
    (- $num:literal) => {
        $crate::Json::from(-$num)
    };
    (- $expr:expr) => {
        $crate::Json::from(-$expr)
    };

    // Simple literal or identifier
    ($other:tt) => {
        $crate::Json::from($other)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pjson_key {
    // Identifier key: name -> "name"
    ($key:ident) => {
        ::std::string::String::from(stringify!($key))
    };
    // String literal key: "my-key" -> "my-key"
    ($key:literal) => {
        ::std::string::String::from($key)
    };
    // Expression key: (expr) -> expr.to_string() or expr if already String
    (($key:expr)) => {
        ::std::string::ToString::to_string(&$key)
    };
}

#[macro_export]
macro_rules! json_array {
    () => {
        $crate::Json::Array(::std::vec::Vec::new())
    };
    ($($elem:expr),* $(,)?) => {
        $crate::Json::Array(::std::vec![ $($crate::Json::from($elem)),* ])
    };
}

#[macro_export]
macro_rules! json_object {
    () => {
        $crate::Json::Object($crate::JsonMap::new())
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = $crate::JsonMap::new();
        $(
            map.insert(::std::string::ToString::to_string(&$key), $crate::Json::from($value));
        )*
        $crate::Json::Object(map)
    }};
}

#[macro_export]
macro_rules! json_value {
    ($value:expr) => {
        $crate::Json::from($value)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pjson_object {
    // End of parsing
    (@parse $map:ident, ) => {};

    // Trailing comma
    (@parse $map:ident, , ) => {};

    // Key-value pair
    (@parse $map:ident, $key:tt : $($rest:tt)* ) => {{
        let k = $crate::pjson_key!($key);
        $crate::pjson_object!(@value $map, k, (), $($rest)*);
    }};

    // Value accumulation until comma
    (@value $map:ident, $k:expr, ($($val:tt)*), , $($tail:tt)* ) => {{
        $map.insert($k, $crate::pjson!($($val)*));
        $crate::pjson_object!(@parse $map, $($tail)*);
    }};

    // Last pair without trailing comma
    (@value $map:ident, $k:expr, ($($val:tt)*), ) => {{
        $map.insert($k, $crate::pjson!($($val)*));
    }};

    // Munch one token at a time
    (@value $map:ident, $k:expr, ($($val:tt)*), $head:tt $($tail:tt)* ) => {{
        $crate::pjson_object!(@value $map, $k, ($($val)* $head), $($tail)*);
    }};
}
