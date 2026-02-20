// projects/libraries/protocol_macros/src/derive_enum_methods.rs
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Type};

use crate::display_mode::DisplayMode;
use crate::enum_utils::{check_snake_case_collisions, to_snake_case};
use crate::parse_display_mode::parse_display_mode;

pub(crate) fn expand_enum_methods(input: DeriveInput) -> proc_macro2::TokenStream {
    // Validate that we're working with an enum
    let enum_data = match &input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(&input, "EnumMethods can only be derived for enums")
                .to_compile_error();
        }
    };

    // Parse attributes to determine display mode (with proper error handling)
    let display_mode = match parse_display_mode(&input.attrs) {
        Ok(mode) => mode,
        Err(e) => return e.to_compile_error(),
    };

    let enum_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Check for snake_case collisions (elite feature!)
    if let Err(e) = check_snake_case_collisions(&enum_data.variants) {
        return e.to_compile_error();
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
    quote! {
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
    }
}
