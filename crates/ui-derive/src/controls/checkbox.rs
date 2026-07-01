use proc_macro2::Ident;
use quote::quote;
use syn::{Field, punctuated::Punctuated, token::Comma};

use super::parse::{is_bool_type, parse_string, snake_to_title_case, variants};

/// Parsed `#[checkbox(...)]` field metadata.
pub struct CheckboxField {
    pub field_name: Ident,
    pub label: String,
}

impl CheckboxField {
    pub fn parse(field: &Field) -> Option<Self> {
        let checkbox_attr = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("checkbox"))?;

        if !is_bool_type(&field.ty) {
            return None;
        }

        let field_name = field.ident.clone()?;
        let mut label: Option<String> = None;
        let nested: Punctuated<syn::Meta, Comma> = checkbox_attr
            .parse_args_with(Punctuated::parse_terminated)
            .ok()?;

        for meta in nested {
            if let syn::Meta::NameValue(nv) = meta {
                let name = nv.path.get_ident()?.to_string();
                if name == "label" {
                    label = Some(parse_string(&nv.value)?);
                }
            }
        }

        let label = label.unwrap_or_else(|| snake_to_title_case(&field_name.to_string()));
        Some(Self { field_name, label })
    }
}

pub fn build_checkbox_enum(
    struct_ident: &Ident,
    fields: &[CheckboxField],
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        return quote! {};
    }

    let enum_ident = Ident::new(
        &format!("{struct_ident}CheckboxControl"),
        struct_ident.span(),
    );
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

pub fn build_checkbox_assoc(
    struct_ident: &Ident,
    fields: &[CheckboxField],
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        return quote! {
            type Checkbox = ::ui_schema::NoControl;
            const CHECKBOXES: &'static [::ui_schema::CheckboxControl<Self::Checkbox>] = &[];
        };
    }

    let enum_ident = Ident::new(
        &format!("{struct_ident}CheckboxControl"),
        struct_ident.span(),
    );
    let variants = variants(fields.iter().map(|f| &f.field_name));
    let labels: Vec<_> = fields.iter().map(|f| f.label.as_str()).collect();

    quote! {
        type Checkbox = #enum_ident;
        const CHECKBOXES: &'static [::ui_schema::CheckboxControl<Self::Checkbox>] = &[
            #(::ui_schema::CheckboxControl {
                id: #enum_ident::#variants,
                label: #labels,
            },)*
        ];
    }
}

pub fn build_checkbox_methods(fields: &[CheckboxField]) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        return quote! {
            fn checkbox_value(_control: Self::Checkbox, _data: &Self) -> bool {
                unreachable!("no checkbox controls are defined for this schema")
            }

            fn set_checkbox(_control: Self::Checkbox, _data: &mut Self, _value: bool) {
                unreachable!("no checkbox controls are defined for this schema")
            }
        };
    }

    let variants = variants(fields.iter().map(|f| &f.field_name));
    let value_arms = fields.iter().zip(variants.iter()).map(|(f, variant)| {
        let field = &f.field_name;
        quote! { Self::Checkbox::#variant => data.#field }
    });
    let set_arms = fields.iter().zip(variants.iter()).map(|(f, variant)| {
        let field = &f.field_name;
        quote! {
            Self::Checkbox::#variant => {
                data.#field = value;
            }
        }
    });

    quote! {
        fn checkbox_value(control: Self::Checkbox, data: &Self) -> bool {
            match control {
                #(#value_arms,)*
            }
        }

        fn set_checkbox(control: Self::Checkbox, data: &mut Self, value: bool) {
            match control {
                #(#set_arms,)*
            }
        }
    }
}
