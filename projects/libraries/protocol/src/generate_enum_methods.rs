/// Macro to generate methods and Display implementation for enums
///
/// This macro simplifies the creation of methods for each variant of an enum.
/// It also generates a `Display` implementation to provide a string representation for each variant.
///
/// # Parameters
/// - `$enum_name`: The name of the enum to implement methods for.
/// - `$snake_case`: The snake_case name of the method to create the variant.
/// - `$pascal_case`: The PascalCase name of the corresponding enum variant.
/// - `$field`: The fields and their types for the variant.
///
/// # Example
/// ```rust
/// generate_enum_methods!(EventVariant,
///     acknowledged => Acknowledged { id: String },
///     created => Created { id: String, data: String },
///     updated => Updated { id: String, old_data: String, new_data: String },
///     deleted => Deleted { id: String },
///     default_variant => Default {},
/// );
/// ```
///
/// This will generate:
/// - Methods like `acknowledged`, `created`, etc., to construct the variants.
/// - A `Display` implementation to format the variants as strings.
#[macro_export]
macro_rules! generate_enum_methods {
    ($enum_name:ident, $($snake_case:ident => $pascal_case:ident { $($field:ident : $type:ty),* $(,)? }),* $(,)?) => {
        impl $enum_name {
            $(
                /// Creates a variant in snake_case
                pub fn $snake_case($($field: $type),*) -> Self {
                    $enum_name::$pascal_case { $($field),* }
                }
            )*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $enum_name::$pascal_case { $($field),* } => {
                            let output = format!("{}: ", stringify!($snake_case));
                            $(write!(f, "{} ", $field)?;)*
                            write!(f, "{}", output.trim_end())
                        }
                    )*
                }
            }
        }
    };
}
