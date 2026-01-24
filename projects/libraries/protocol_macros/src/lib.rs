// projects/libraries/protocol_macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Type, parse_macro_input};

#[proc_macro_derive(EnumMethods, attributes(enum_methods))]
pub fn derive_enum_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Validate that we're working with an enum
    let enum_data = match &input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(&input, "EnumMethods can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    // Parse attributes to determine display mode (with proper error handling)
    let display_mode = match parse_display_mode(&input.attrs) {
        Ok(mode) => mode,
        Err(e) => return e.to_compile_error().into(),
    };

    let enum_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Check for snake_case collisions (elite feature!)
    if let Err(e) = check_snake_case_collisions(&enum_data.variants) {
        return e.to_compile_error().into();
    }

    // Generate constructor methods
    let constructors = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let snake_name = to_snake_case(&variant_name.to_string());
        let snake_ident = Ident::new(&snake_name, variant_name.span());

        match &variant.fields {
            // Unit variant: Ping
            Fields::Unit => {
                quote! {
                    #[inline]
                    pub fn #snake_ident() -> Self {
                        Self::#variant_name
                    }
                }
            }
            // Struct variant: Created { id: String, data: String }
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().expect("field ident"))
                    .collect();
                let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();

                quote! {
                    #[inline]
                    pub fn #snake_ident(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name { #(#field_names),* }
                    }
                }
            }
            // Tuple variant: Data(String, u32)
            Fields::Unnamed(fields) => {
                let arg_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("arg{}", i), variant_name.span()))
                    .collect();
                let arg_types: Vec<&Type> = fields.unnamed.iter().map(|f| &f.ty).collect();

                quote! {
                    #[inline]
                    pub fn #snake_ident(#(#arg_names: #arg_types),*) -> Self {
                        Self::#variant_name(#(#arg_names),*)
                    }
                }
            }
        }
    });

    // Generate Display implementation
    let display_arms = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let snake = to_snake_case(&variant_name.to_string());
        let snake_lit = LitStr::new(&snake, variant_name.span());

        match &variant.fields {
            // Unit variant: "ping"
            Fields::Unit => {
                quote! {
                    Self::#variant_name => f.write_str(#snake_lit)
                }
            }
            // Struct variant: "created { id=..., data=... }"
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().expect("field ident"))
                    .collect();

                let format_fields = if field_names.is_empty() {
                    quote! {}
                } else {
                    let field_formats = field_names.iter().enumerate().map(|(i, name)| {
                        let name_str = name.to_string();
                        if i == 0 {
                            match display_mode {
                                DisplayMode::Display => quote! {
                                    write!(f, "{}={}", #name_str, #name)?;
                                },
                                DisplayMode::Debug => quote! {
                                    write!(f, "{}={:?}", #name_str, #name)?;
                                },
                            }
                        } else {
                            match display_mode {
                                DisplayMode::Display => quote! {
                                    write!(f, ", {}={}", #name_str, #name)?;
                                },
                                DisplayMode::Debug => quote! {
                                    write!(f, ", {}={:?}", #name_str, #name)?;
                                },
                            }
                        }
                    });
                    quote! { #(#field_formats)* }
                };

                quote! {
                    Self::#variant_name { #(#field_names),* } => {
                        f.write_str(#snake_lit)?;
                        f.write_str(" { ")?;
                        #format_fields
                        f.write_str(" }")
                    }
                }
            }
            // Tuple variant: "data(arg0=..., arg1=...)"
            Fields::Unnamed(fields) => {
                let arg_names: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("arg{}", i), variant_name.span()))
                    .collect();

                let format_args = if arg_names.is_empty() {
                    quote! {}
                } else {
                    let arg_formats = arg_names.iter().enumerate().map(|(i, name)| {
                        let name_str = format!("arg{}", i);
                        if i == 0 {
                            match display_mode {
                                DisplayMode::Display => quote! {
                                    write!(f, "{}={}", #name_str, #name)?;
                                },
                                DisplayMode::Debug => quote! {
                                    write!(f, "{}={:?}", #name_str, #name)?;
                                },
                            }
                        } else {
                            match display_mode {
                                DisplayMode::Display => quote! {
                                    write!(f, ", {}={}", #name_str, #name)?;
                                },
                                DisplayMode::Debug => quote! {
                                    write!(f, ", {}={:?}", #name_str, #name)?;
                                },
                            }
                        }
                    });
                    quote! { #(#arg_formats)* }
                };

                quote! {
                    Self::#variant_name(#(#arg_names),*) => {
                        f.write_str(#snake_lit)?;
                        f.write_str("(")?;
                        #format_args
                        f.write_str(")")
                    }
                }
            }
        }
    });

    // Generate as_str() method for each variant (premium feature!)
    let as_str_arms = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let snake = to_snake_case(&variant_name.to_string());
        let snake_lit = LitStr::new(&snake, variant_name.span());

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    Self::#variant_name => #snake_lit
                }
            }
            Fields::Named(_) => {
                quote! {
                    Self::#variant_name { .. } => #snake_lit
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    Self::#variant_name(..) => #snake_lit
                }
            }
        }
    });

    // Generate the complete implementation
    let expanded = quote! {
        impl #impl_generics #enum_name #ty_generics #where_clause {
            #(#constructors)*

            /// Returns the variant name as a static string slice
            ///
            /// This is useful for:
            /// - Logging and debugging
            /// - Routing and dispatch
            /// - Serialization keys
            /// - Pattern matching on variant names
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(#as_str_arms),*
                }
            }
        }

        impl #impl_generics ::std::fmt::Display for #enum_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Display formatting mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayMode {
    Display, // Uses {} formatting
    Debug,   // Uses {:?} formatting
}

/// Parse display mode from attributes with proper error handling
fn parse_display_mode(attrs: &[syn::Attribute]) -> Result<DisplayMode, syn::Error> {
    for attr in attrs {
        if !attr.path().is_ident("enum_methods") {
            continue;
        }

        let mut mode: Option<String> = None;

        // parse_nested_meta provides reliable parsing and proper error messages
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("mode") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                mode = Some(lit.value());
                Ok(())
            } else {
                Err(meta.error("unknown enum_methods attribute, expected `mode`"))
            }
        })?;

        if let Some(m) = mode.as_deref() {
            if m.eq_ignore_ascii_case("debug") {
                return Ok(DisplayMode::Debug);
            } else {
                return Err(syn::Error::new_spanned(
                    &attr.meta,
                    format!("unknown mode `{m}`, expected `debug`"),
                ));
            }
        }
    }

    Ok(DisplayMode::Display)
}

/// Check for snake_case naming collisions (elite feature!)
///
/// This prevents subtle bugs where two variants with different PascalCase names
/// generate the same snake_case method name.
///
/// Example collision:
/// - `HTTPServer` → `http_server`
/// - `HttpServer` → `http_server`  // Collision!
fn check_snake_case_collisions(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Result<(), syn::Error> {
    let mut seen: HashMap<String, &Ident> = HashMap::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let snake = to_snake_case(&variant_name.to_string());

        if let Some(existing) = seen.get(&snake) {
            let existing = existing.to_string();
            let current = variant_name.to_string();
            return Err(syn::Error::new_spanned(
                variant_name,
                format!(
                    "snake_case name collision: variant `{current}` generates method `{snake}` already used by `{existing}`\n\
                     hint: rename one of these variants"
                ),
            ));
        }

        seen.insert(snake, variant_name);
    }

    Ok(())
}

/// Convert PascalCase to snake_case (handles acronyms correctly)
///
/// Examples:
/// - `HTTPRequest` → `http_request`
/// - `HTTPServerError` → `http_server_error`
/// - `DataReceived` → `data_received`
/// - `Ping` → `ping`
fn to_snake_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    // Optimal capacity: original length + estimated number of underscores
    let uppercase_count = chars.iter().filter(|c| c.is_ascii_uppercase()).count();
    let mut out = String::with_capacity(s.len() + uppercase_count);

    for i in 0..chars.len() {
        let c = chars[i];
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = chars.get(i + 1).copied();

        let is_upper = c.is_ascii_uppercase();
        let prev_is_lower = prev.map(|p| p.is_ascii_lowercase()).unwrap_or(false);
        let prev_is_upper = prev.map(|p| p.is_ascii_uppercase()).unwrap_or(false);
        let prev_is_digit = prev.map(|p| p.is_ascii_digit()).unwrap_or(false);
        let next_is_lower = next.map(|n| n.is_ascii_lowercase()).unwrap_or(false);

        // Add underscore when:
        // 1) Transitioning from lowercase to uppercase (dataReceived)
        // 2) End of acronym: uppercase followed by lowercase with previous uppercase (HTTPRequest)
        // 3) Transitioning from digit to uppercase letter (HTTP2Request)
        if i > 0 && is_upper && (prev_is_lower || prev_is_digit || (prev_is_upper && next_is_lower))
        {
            out.push('_');
        }

        out.push(c.to_ascii_lowercase());
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        // Basic cases
        assert_eq!(to_snake_case("Ping"), "ping");
        assert_eq!(to_snake_case("Created"), "created");

        // CamelCase to snake_case
        assert_eq!(to_snake_case("DataReceived"), "data_received");
        assert_eq!(to_snake_case("UserLoggedIn"), "user_logged_in");

        // Acronym handling (premium feature!)
        assert_eq!(to_snake_case("HTTPRequest"), "http_request");
        assert_eq!(to_snake_case("HTTPServerError"), "http_server_error");
        assert_eq!(to_snake_case("XMLParser"), "xml_parser");
        assert_eq!(to_snake_case("JSONData"), "json_data");

        // Edge cases
        assert_eq!(to_snake_case("A"), "a");
        assert_eq!(to_snake_case("AB"), "ab");
        assert_eq!(to_snake_case("ABC"), "abc");
        assert_eq!(to_snake_case("ABCDef"), "abc_def");

        // Numbers in identifiers (regression prevention)
        assert_eq!(to_snake_case("V2"), "v2");
        assert_eq!(to_snake_case("HTTP2Request"), "http2_request");
        assert_eq!(to_snake_case("Version2Alpha"), "version2_alpha");
        assert_eq!(to_snake_case("Base64Encoder"), "base64_encoder");
        assert_eq!(to_snake_case("UTF8String"), "utf8_string");
    }
}
