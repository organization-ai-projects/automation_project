// projects/libraries/protocol_macros/src/parse_display_mode.rs
use syn::LitStr;

use crate::display_mode::DisplayMode;

/// Parse display mode from attributes with proper error handling
pub(crate) fn parse_display_mode(attrs: &[syn::Attribute]) -> Result<DisplayMode, syn::Error> {
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
