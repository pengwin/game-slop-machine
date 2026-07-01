//! Derive macros for UI control schemas.

mod controls;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro that generates control metadata and accessors.
///
/// # Example
///
/// ```ignore
/// #[derive(Controls)]
/// #[controls(post = "normalize")]
/// pub struct Params {
///     #[slider(min = 0.0, max = 9999.0, step = 1.0, precision = 0)]
///     pub seed: u32,
///
///     #[slider(min = 0.0, max = 0.3)]
///     pub tone_variation: f32,
///
///     #[checkbox(label = "Enabled")]
///     pub enabled: bool,
/// }
/// ```
#[proc_macro_derive(Controls, attributes(controls, slider, checkbox))]
pub fn derive_controls(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        controls::impl_controls(&input, data)
    } else {
        TokenStream::from(
            syn::Error::new(input.ident.span(), "Only structs can derive `Controls`")
                .to_compile_error(),
        )
    }
}
