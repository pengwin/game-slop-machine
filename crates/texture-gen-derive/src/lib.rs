//! Derive macros for texture-gen.

mod sliders;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro that generates slider enum and methods from `#[slider(...)]` attributes.
///
/// # Example
///
/// ```ignore
/// #[derive(Sliders)]
/// pub struct ConcreteParams {
///     #[slider(min = 0.0, max = 9999.0, step = 1.0, precision = 0)]
///     pub seed: u32,
///
///     #[slider(min = 0.0, max = 0.3)]
///     pub tone_variation: f32,
/// }
/// ```
#[proc_macro_derive(Sliders, attributes(slider))]
pub fn derive_sliders(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        sliders::impl_sliders(&input.ident, data)
    } else {
        TokenStream::from(
            syn::Error::new(input.ident.span(), "Only structs can derive `Sliders`").to_compile_error(),
        )
    }
}
