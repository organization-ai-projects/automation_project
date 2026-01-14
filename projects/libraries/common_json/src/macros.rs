//! Macros de construction JSON.
//!
//! Ce module fournit des macros pour créer du JSON de manière déclarative,
//! avec des fonctionnalités supplémentaires.

#[macro_export]
macro_rules! pjson {
    //================================================================
    // NULL
    //================================================================
    (null) => {
        $crate::Json::Null
    };

    //================================================================
    // BOOLEANS
    //================================================================
    (true) => {
        $crate::Json::Bool(true)
    };
    (false) => {
        $crate::Json::Bool(false)
    };

    //================================================================
    // ARRAYS
    //================================================================
    ([]) => {
        $crate::Json::Array(vec![])
    };
    ([ $($elem:tt),* $(,)? ]) => {
        $crate::Json::Array(vec![ $($crate::pjson!($elem)),* ])
    };

    //================================================================
    // OBJECTS
    //================================================================
    ({}) => {
        $crate::Json::Object($crate::JsonMap::new())
    };
    ({ $($key:tt : $value:tt),* $(,)? }) => {
        {
            let mut map = $crate::JsonMap::new();
            $(
                map.insert($crate::pjson_key!($key).to_string(), $crate::pjson!($value));
            )*
            $crate::Json::Object(map)
        }
    };

    //================================================================
    // PRIMITIVES (numbers, strings, expressions)
    //================================================================
    ($other:expr) => {
        $crate::serialize::to_json(&$other).expect("JSON valide")
    };
}

/// Helper macro to handle different key types in objects.
///
/// Supports:
/// - Identifiers: `name` -> `"name"`
/// - String literals: `"key-name"` -> `"key-name"`
/// - Expressions in parentheses: `(my_var)` -> value of `my_var`
#[macro_export]
#[doc(hidden)]
macro_rules! pjson_key {
    // Identifier key: name -> "name"
    ($key:ident) => {
        stringify!($key)
    };
    // String literal key: "my-key" -> "my-key"
    ($key:literal) => {
        $key
    };
    // Expression key: (expr) -> expr
    (($key:expr)) => {
        $key
    };
}

/// Create a JSON array from values.
///
/// # Examples
///
/// ```
/// use common_json::json_array;
///
/// let arr = json_array![1, 2, 3];
/// let mixed = json_array!["hello", 42, true];
/// ```
#[macro_export]
macro_rules! json_array {
    () => {
        $crate::Json::Array(vec![])
    };
    ($($elem:expr),* $(,)?) => {
        $crate::Json::Array(vec![ $($crate::serialize::to_json(&$elem).expect("JSON valide")),* ])
    };
}

/// Create a JSON object from key-value pairs.
///
/// # Examples
///
/// ```
/// use common_json::json_object;
///
/// let obj = json_object! {
///     "name" => "Alice",
///     "age" => 30,
/// };
/// ```
#[macro_export]
macro_rules! json_object {
    () => {
        $crate::Json::Object($crate::JsonMap::new())
    };
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = $crate::JsonMap::new();
            $(
                map.insert($key.to_string(), $crate::serialize::to_json(&$value).expect("JSON valide"));
            )*
            $crate::Json::Object(map)
        }
    };
}
