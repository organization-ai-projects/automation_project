// projects/libraries/protocol_macros/src/enum_utils.rs
use std::collections::HashMap;

use syn::Ident;

/// Check for snake_case naming collisions (elite feature!)
///
/// This prevents subtle bugs where two variants with different PascalCase names
/// generate the same snake_case method name.
///
/// Example collision:
/// - `HTTPServer` → `http_server`
/// - `HttpServer` → `http_server`  // Collision!
pub(crate) fn check_snake_case_collisions(
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
pub(crate) fn to_snake_case(s: &str) -> String {
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
