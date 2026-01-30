// projects/libraries/protocol_macros/src/lib.rs

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derive_enum_methods;
mod display_mode;
mod enum_utils;
mod parse_display_mode;

use derive_enum_methods::expand_enum_methods;

#[proc_macro_derive(EnumMethods, attributes(enum_methods))]
pub fn derive_enum_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(expand_enum_methods(input))
}

// parse_display_mode moved to module for clarity

#[cfg(test)]
mod tests;
