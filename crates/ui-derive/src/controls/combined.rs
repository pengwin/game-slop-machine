use proc_macro2::Ident;
use quote::quote;

use super::{checkbox::CheckboxField, parse::variants, slider::SliderField};

/// Control kind plus index into that kind's parsed metadata list.
pub enum ControlField {
    Slider(usize),
    Checkbox(usize),
}

pub fn build_control_assoc(
    struct_ident: &Ident,
    sliders: &[SliderField],
    checkboxes: &[CheckboxField],
    controls: &[ControlField],
) -> proc_macro2::TokenStream {
    if controls.is_empty() {
        return quote! {
            const CONTROLS: &'static [::ui_schema::Control<Self::Slider, Self::Checkbox>] = &[];
        };
    }

    let slider_enum = Ident::new(&format!("{struct_ident}SliderControl"), struct_ident.span());
    let checkbox_enum = Ident::new(
        &format!("{struct_ident}CheckboxControl"),
        struct_ident.span(),
    );
    let slider_variants = variants(sliders.iter().map(|f| &f.field_name));
    let checkbox_variants = variants(checkboxes.iter().map(|f| &f.field_name));
    let entries = controls.iter().map(|control| match control {
        ControlField::Slider(index) => {
            let field = &sliders[*index];
            let variant = &slider_variants[*index];
            let label = field.label.as_str();
            let min = field.min;
            let max = field.max;
            let step = field.step;
            let precision = field.precision;
            quote! {
                ::ui_schema::Control::Slider(::ui_schema::SliderControl {
                    id: #slider_enum::#variant,
                    label: #label,
                    min: #min,
                    max: #max,
                    step: #step,
                    precision: #precision,
                })
            }
        }
        ControlField::Checkbox(index) => {
            let field = &checkboxes[*index];
            let variant = &checkbox_variants[*index];
            let label = field.label.as_str();
            quote! {
                ::ui_schema::Control::Checkbox(::ui_schema::CheckboxControl {
                    id: #checkbox_enum::#variant,
                    label: #label,
                })
            }
        }
    });

    quote! {
        const CONTROLS: &'static [::ui_schema::Control<Self::Slider, Self::Checkbox>] = &[
            #(#entries,)*
        ];
    }
}
