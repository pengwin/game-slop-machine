use proc_macro2::Ident;
use quote::quote;
use syn::{Field, punctuated::Punctuated, token::Comma};

use super::parse::{
    is_u32_type, parse_f32, parse_i32, parse_string, snake_to_title_case, variants,
};

/// Parsed `#[slider(...)]` field metadata.
pub struct SliderField {
    pub field_name: Ident,
    pub label: String,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub precision: i32,
    is_u32: bool,
}

impl SliderField {
    pub fn parse(field: &Field) -> Option<Self> {
        let slider_attr = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("slider"))?;

        let field_name = field.ident.clone()?;
        let is_u32 = is_u32_type(&field.ty);

        let mut min: Option<f32> = None;
        let mut max: Option<f32> = None;
        let mut step: Option<f32> = None;
        let mut precision: Option<i32> = None;
        let mut label: Option<String> = None;

        let nested: Punctuated<syn::Meta, Comma> = slider_attr
            .parse_args_with(Punctuated::parse_terminated)
            .ok()?;

        for meta in nested {
            if let syn::Meta::NameValue(nv) = meta {
                let name = nv.path.get_ident()?.to_string();
                match name.as_str() {
                    "min" => min = Some(parse_f32(&nv.value)?),
                    "max" => max = Some(parse_f32(&nv.value)?),
                    "step" => step = Some(parse_f32(&nv.value)?),
                    "precision" => precision = Some(parse_i32(&nv.value)?),
                    "label" => label = Some(parse_string(&nv.value)?),
                    _ => {}
                }
            }
        }

        let min = min?;
        let max = max?;
        let step = step.unwrap_or_else(|| (max - min) / 100.0);
        let precision = precision.unwrap_or(2);
        let label = label.unwrap_or_else(|| snake_to_title_case(&field_name.to_string()));

        Some(Self {
            field_name,
            label,
            min,
            max,
            step,
            precision,
            is_u32,
        })
    }
}

pub fn build_slider_enum(
    struct_ident: &Ident,
    fields: &[SliderField],
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        return quote! {};
    }

    let enum_ident = Ident::new(&format!("{struct_ident}SliderControl"), struct_ident.span());
    let variants = variants(fields.iter().map(|f| &f.field_name));

    quote! {
        #[derive(Clone, Copy, Default)]
        #[allow(missing_docs)]
        pub enum #enum_ident {
            #[default]
            #(#variants,)*
        }
    }
}

pub fn build_slider_assoc(
    struct_ident: &Ident,
    fields: &[SliderField],
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        return quote! {
            type Slider = ::ui_schema::NoControl;
            const SLIDERS: &'static [::ui_schema::SliderControl<Self::Slider>] = &[];
        };
    }

    let enum_ident = Ident::new(&format!("{struct_ident}SliderControl"), struct_ident.span());
    let variants = variants(fields.iter().map(|f| &f.field_name));
    let labels: Vec<_> = fields.iter().map(|f| f.label.as_str()).collect();
    let mins: Vec<_> = fields.iter().map(|f| f.min).collect();
    let maxs: Vec<_> = fields.iter().map(|f| f.max).collect();
    let steps: Vec<_> = fields.iter().map(|f| f.step).collect();
    let precisions: Vec<_> = fields.iter().map(|f| f.precision).collect();

    quote! {
        type Slider = #enum_ident;
        const SLIDERS: &'static [::ui_schema::SliderControl<Self::Slider>] = &[
            #(::ui_schema::SliderControl {
                id: #enum_ident::#variants,
                label: #labels,
                min: #mins,
                max: #maxs,
                step: #steps,
                precision: #precisions,
            },)*
        ];
    }
}

pub fn build_slider_methods(
    fields: &[SliderField],
    post_fn: Option<&str>,
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        return quote! {
            fn slider_value(_control: Self::Slider, _data: &Self) -> f32 {
                unreachable!("no slider controls are defined for this schema")
            }

            fn set_slider(_control: Self::Slider, _data: &mut Self, _value: f32) {
                unreachable!("no slider controls are defined for this schema")
            }
        };
    }

    let variants = variants(fields.iter().map(|f| &f.field_name));
    let value_arms = fields.iter().zip(variants.iter()).map(|(f, variant)| {
        let field = &f.field_name;
        if f.is_u32 {
            quote! { Self::Slider::#variant => data.#field as f32 }
        } else {
            quote! { Self::Slider::#variant => data.#field }
        }
    });
    let set_arms = fields.iter().zip(variants.iter()).map(|(f, variant)| {
        let field = &f.field_name;
        let min = f.min;
        let max = f.max;
        if f.is_u32 {
            quote! {
                Self::Slider::#variant => {
                    data.#field = value.round().clamp(#min, #max) as u32;
                }
            }
        } else {
            quote! {
                Self::Slider::#variant => {
                    data.#field = value.clamp(#min, #max);
                }
            }
        }
    });
    let post_call = post_fn.map(|fn_name| {
        let fn_ident = Ident::new(fn_name, proc_macro2::Span::call_site());
        quote! { data.#fn_ident(); }
    });

    quote! {
        fn slider_value(control: Self::Slider, data: &Self) -> f32 {
            match control {
                #(#value_arms,)*
            }
        }

        fn set_slider(control: Self::Slider, data: &mut Self, value: f32) {
            match control {
                #(#set_arms,)*
            }
            #post_call
        }
    }
}
