//! Macros de construction JSON optimisées.
//!
//! Ce module fournit des macros pour créer du JSON de manière déclarative,
//! avec des performances optimales et une ergonomie supérieure à `serde_json::json!`.
//!
//! Pour plus de détails, exemples et documentation, consultez le fichier `docs/macros.md`.

/// Macro principale pour créer des valeurs JSON.
///
/// Supporte tous les types JSON : null, bool, number, string, array, object.
/// Utilise `Into<Json>` pour les conversions.
///
/// # Fonctionnalités
///
/// - **Expressions complexes** : appels de méthodes, arithmétique, etc.
/// - **Variables directes** : `pjson!(my_var)` sans conversion explicite
/// - **Types automatiques** : `Option<T>`, `Vec<T>`, tous les entiers, `&str`, `String`, etc.
/// - **Clés flexibles** : identifiant `name` → `"name"`, ou littéral `"my-key"`
/// - **Nested** : objets et tableaux imbriqués
///
/// # Limitation
///
/// **Turbofish** : les expressions avec `::<A, B>` doivent être wrappées dans `( )` :
/// ```rust,ignore
/// pjson!({ "x": (foo::<i32, u32>()) })  // OK
/// pjson!({ "x": foo::<i32, u32>() })    // Erreur de compilation
/// ```
///
/// # Exemples
///
/// ```rust
/// use common_json::pjson;
///
/// let path = std::path::Path::new("/tmp/test");
/// let obj = pjson!({
///     "simple": 42,
///     "method_call": path.to_string_lossy().to_string(),
///     "arithmetic": 1 + 2 * 3,
///     nested: {  // clé identifiant → "nested"
///         "array": [1, 2, 3]
///     }
/// });
/// ```
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

/// Helper macro pour convertir les valeurs dans les objets.
///
/// Gère les cas spéciaux :
/// - `null`, `true`, `false` → valeurs JSON directes
/// - `[...]` → tableaux JSON (récursif)
/// - `{...}` → objets JSON (récursif)
/// - `(expr)` → expressions complexes (appels de méthodes, etc.)
/// - littéraux et identifiants → conversion via Into<Json>
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

/// Helper macro pour gérer les différents types de clés dans les objets.
///
/// Supporte :
/// - Identifiants : `name` → `"name"`
/// - Littéraux string : `"key-name"` → `"key-name"`
/// - Expressions entre parenthèses : `(my_var)` → valeur de `my_var`
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

/// Crée un tableau JSON à partir de valeurs.
///
/// Utilise `Into<Json>` pour chaque élément, permettant des types mixtes
/// tant qu'ils implémentent `Into<Json>`.
///
/// # Exemples
///
/// ```rust
/// use common_json::json_array;
///
/// let arr = json_array![1, 2, 3];
/// let mixed = json_array!["hello", 42, true];
///
/// let name = "test";
/// let age = 25;
/// let active = true;
/// let from_vars = json_array![name, age, active];
/// ```
#[macro_export]
macro_rules! json_array {
    () => {
        $crate::Json::Array(::std::vec::Vec::new())
    };
    ($($elem:expr),* $(,)?) => {
        $crate::Json::Array(::std::vec![ $($crate::Json::from($elem)),* ])
    };
}

/// Crée un objet JSON à partir de paires clé-valeur.
///
/// Utilise la syntaxe `=>` pour séparer clés et valeurs.
/// Les clés sont converties en `String`, les valeurs via `Into<Json>`.
///
/// # Exemples
///
/// ```rust
/// use common_json::json_object;
///
/// let obj = json_object! {
///     "name" => "Alice",
///     "age" => 30,
/// };
///
/// // Avec des variables
/// let key = "dynamic";
/// let value = 42;
/// let obj = json_object! {
///     key => value,
///     "static" => "value",
/// };
/// ```
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

/// Macro pour créer un `Json` à partir d'une expression quelconque.
///
/// Cette macro est un raccourci pour `Json::from(expr)` et fonctionne
/// avec tout type implémentant `Into<Json>`.
///
/// # Exemples
///
/// ```rust
/// use common_json::json_value;
///
/// let s = json_value!("hello");
/// let n = json_value!(42);
/// let b = json_value!(true);
/// let opt = json_value!(Some(10));
/// let none = json_value!(None::<i32>);
/// ```
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

#[cfg(test)]
mod tests {
    use crate::Json;

    #[test]
    fn test_pjson_complex_expressions_without_parens() {
        // Test: expressions complexes SANS parenthèses
        let path = std::path::Path::new("/tmp/test");

        let obj = pjson!({
            "simple": 42,
            "method_call": path.to_string_lossy().to_string(),
            "chained": "hello".to_uppercase(),
            "arithmetic": 1 + 2 * 3
        });

        assert!(obj.is_object());
        let map = obj.as_object().unwrap();
        assert_eq!(map.get("simple"), Some(&Json::from(42)));
        assert_eq!(
            map.get("method_call"),
            Some(&Json::from(path.to_string_lossy().to_string()))
        );
        assert_eq!(map.get("chained"), Some(&Json::from("HELLO")));
        assert_eq!(map.get("arithmetic"), Some(&Json::from(7)));
    }

    #[test]
    fn test_pjson_nested_objects_and_arrays() {
        let name = "Alice";
        let age = 30;

        let obj = pjson!({
            "user": {
                "name": name,
                "age": age,
                "computed": age * 2
            },
            "tags": ["admin", "user"],
            "scores": [1, 2, 3]
        });

        assert!(obj.is_object());
    }

    #[test]
    fn test_pjson_all_types() {
        let obj = pjson!({
            "null_val": null,
            "bool_true": true,
            "bool_false": false,
            "negative": -42,
            "float": std::f64::consts::PI,
            "string": "hello",
            "array": [1, 2, 3],
            "nested": { "a": 1 }
        });

        let map = obj.as_object().unwrap();
        assert_eq!(map.get("null_val"), Some(&Json::Null));
        assert_eq!(map.get("bool_true"), Some(&Json::Bool(true)));
        assert_eq!(map.get("bool_false"), Some(&Json::Bool(false)));
    }

    #[test]
    fn test_pjson_variables_direct() {
        let x = 42;
        let s = "test";
        let v = vec![1, 2, 3];

        // Variables directes
        assert_eq!(pjson!(x), Json::from(42));
        assert_eq!(pjson!(s), Json::from("test"));

        // Vec via From
        let arr = pjson!(v);
        assert!(arr.is_array());
    }

    #[test]
    fn test_json_array_macro() {
        let arr = json_array![1, 2, 3];
        assert!(arr.is_array());
        assert_eq!(arr.as_array().unwrap().len(), 3);

        // Mixed types
        let mixed = json_array!["hello", 42, true];
        assert_eq!(mixed.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_json_object_macro() {
        let obj = json_object! {
            "name" => "Alice",
            "age" => 30,
        };

        let map = obj.as_object().unwrap();
        assert_eq!(map.get("name"), Some(&Json::from("Alice")));
        assert_eq!(map.get("age"), Some(&Json::from(30)));
    }

    #[test]
    fn test_pjson_ident_keys() {
        // Clés identifiants → stringify automatique
        let obj = pjson!({
            name: "Alice",
            age: 30,
            active: true
        });

        let map = obj.as_object().unwrap();
        assert_eq!(map.get("name"), Some(&Json::from("Alice")));
        assert_eq!(map.get("age"), Some(&Json::from(30)));
        assert_eq!(map.get("active"), Some(&Json::Bool(true)));
    }

    #[test]
    fn test_pjson_dynamic_key() {
        // Clé dynamique via (expr)
        let key_name = "dynamic_key";
        let obj = pjson!({
            (key_name): 42
        });

        let map = obj.as_object().unwrap();
        assert_eq!(map.get("dynamic_key"), Some(&Json::from(42)));
    }
}
