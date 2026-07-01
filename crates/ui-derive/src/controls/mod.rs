mod checkbox;
mod combined;
mod parse;
mod slider;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput};

use checkbox::{CheckboxField, build_checkbox_assoc, build_checkbox_enum, build_checkbox_methods};
use combined::{ControlField, build_control_assoc};
use parse::parse_post_attr;
use slider::{SliderField, build_slider_assoc, build_slider_enum, build_slider_methods};

/// Generates the `ControlsSchema` implementation.
pub fn impl_controls(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let struct_ident = &input.ident;
    let post_fn = parse_post_attr(input);

    let mut sliders = Vec::new();
    let mut checkboxes = Vec::new();
    let mut controls = Vec::new();

    for field in &data.fields {
        if let Some(slider) = SliderField::parse(field) {
            controls.push(ControlField::Slider(sliders.len()));
            sliders.push(slider);
        } else if let Some(checkbox) = CheckboxField::parse(field) {
            controls.push(ControlField::Checkbox(checkboxes.len()));
            checkboxes.push(checkbox);
        }
    }

    let slider_enum = build_slider_enum(struct_ident, &sliders);
    let checkbox_enum = build_checkbox_enum(struct_ident, &checkboxes);
    let slider_assoc = build_slider_assoc(struct_ident, &sliders);
    let checkbox_assoc = build_checkbox_assoc(struct_ident, &checkboxes);
    let control_assoc = build_control_assoc(struct_ident, &sliders, &checkboxes, &controls);
    let slider_methods = build_slider_methods(&sliders, post_fn.as_deref());
    let checkbox_methods = build_checkbox_methods(&checkboxes);

    TokenStream::from(quote! {
        #slider_enum
        #checkbox_enum

        #[allow(clippy::use_self)]
        impl ::ui_schema::ControlsSchema for #struct_ident {
            #slider_assoc
            #checkbox_assoc
            #control_assoc
            #slider_methods
            #checkbox_methods
        }
    })
}
