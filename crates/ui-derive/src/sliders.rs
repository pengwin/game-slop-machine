use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataStruct, DeriveInput, Expr, Field, Lit, Type, punctuated::Punctuated, token::Comma};

struct SliderField {
    field_name: Ident,
    label: String,
    min: f32,
    max: f32,
    step: f32,
    precision: i32,
    is_u32: bool,
}

pub fn impl_sliders(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let struct_ident = &input.ident;
    let post_fn = parse_post_attr(input);

    let fields: Vec<SliderField> = data
        .fields
        .iter()
        .filter_map(parse_slider_field)
        .collect();

    if fields.is_empty() {
        return TokenStream::from(quote! {});
    }

    let enum_ident = Ident::new(&format!("{struct_ident}Slider"), struct_ident.span());
    let variants: Vec<Ident> = fields
        .iter()
        .map(|f| Ident::new(&snake_to_pascal_case(&f.field_name.to_string()), f.field_name.span()))
        .collect();

    let value_arms = build_value_arms(&fields, &variants);
    let set_arms = build_set_arms(&fields, &variants);
    let post_call = build_post_call(struct_ident, post_fn);

    generate_enum(struct_ident, &enum_ident, &variants, &fields, &value_arms, &set_arms, post_call.as_ref())
}

fn build_value_arms(fields: &[SliderField], variants: &[Ident]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .zip(variants.iter())
        .map(|(f, variant)| {
            let field = &f.field_name;
            if f.is_u32 {
                quote! { Self::#variant => params.#field as f32 }
            } else {
                quote! { Self::#variant => params.#field }
            }
        })
        .collect()
}

fn build_set_arms(fields: &[SliderField], variants: &[Ident]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .zip(variants.iter())
        .map(|(f, variant)| {
            let field = &f.field_name;
            let min = f.min;
            let max = f.max;
            if f.is_u32 {
                quote! {
                    Self::#variant => {
                        params.#field = value.round().clamp(#min, #max) as u32;
                    }
                }
            } else {
                quote! {
                    Self::#variant => {
                        params.#field = value.clamp(#min, #max);
                    }
                }
            }
        })
        .collect()
}

fn build_post_call(
    struct_ident: &Ident,
    post_fn: Option<String>,
) -> Option<proc_macro2::TokenStream> {
    let fn_name = post_fn?;
    let fn_ident = Ident::new(&fn_name, struct_ident.span());
    Some(quote! { params.#fn_ident(); })
}

fn generate_enum(
    struct_ident: &Ident,
    enum_ident: &Ident,
    variants: &[Ident],
    fields: &[SliderField],
    value_arms: &[proc_macro2::TokenStream],
    set_arms: &[proc_macro2::TokenStream],
    post_call: Option<&proc_macro2::TokenStream>,
) -> TokenStream {
    let labels: Vec<_> = fields.iter().map(|f| f.label.as_str()).collect();
    let mins: Vec<_> = fields.iter().map(|f| f.min).collect();
    let maxs: Vec<_> = fields.iter().map(|f| f.max).collect();
    let steps: Vec<_> = fields.iter().map(|f| f.step).collect();
    let precisions: Vec<_> = fields.iter().map(|f| f.precision).collect();

    let expanded = quote! {
        #[derive(Clone, Default)]
        #[allow(missing_docs)]
        pub enum #enum_ident {
            #[default]
            #(#variants,)*
        }

        #[allow(missing_docs)]
        impl #enum_ident {
            pub fn label(&self) -> &'static str {
                match self {
                    #(Self::#variants => #labels,)*
                }
            }

            pub fn min(&self) -> f32 {
                match self {
                    #(Self::#variants => #mins,)*
                }
            }

            pub fn max(&self) -> f32 {
                match self {
                    #(Self::#variants => #maxs,)*
                }
            }

            pub fn step(&self) -> f32 {
                match self {
                    #(Self::#variants => #steps,)*
                }
            }

            pub fn precision(&self) -> i32 {
                match self {
                    #(Self::#variants => #precisions,)*
                }
            }

            pub fn value(&self, params: &#struct_ident) -> f32 {
                match self {
                    #(#value_arms,)*
                }
            }

            pub fn set(&self, params: &mut #struct_ident, value: f32) {
                match self {
                    #(#set_arms,)*
                }
                #post_call
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_post_attr(input: &DeriveInput) -> Option<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("slider") {
            continue;
        }

        let nested: Punctuated<syn::Meta, Comma> = attr.parse_args_with(Punctuated::parse_terminated).ok()?;

        for meta in nested {
            if let syn::Meta::NameValue(nv) = meta
                && nv.path.is_ident("post")
            {
                return parse_string(&nv.value);
            }
        }
    }
    None
}

fn parse_slider_field(field: &Field) -> Option<SliderField> {
    let slider_attr = field.attrs.iter().find(|attr| attr.path().is_ident("slider"))?;

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

    Some(SliderField {
        field_name,
        label,
        min,
        max,
        step,
        precision,
        is_u32,
    })
}

fn parse_f32(expr: &Expr) -> Option<f32> {
    match expr {
        Expr::Lit(lit) => {
            if let Lit::Float(f) = &lit.lit {
                return f.base10_parse().ok();
            }
            if let Lit::Int(i) = &lit.lit {
                return i.base10_parse::<f32>().ok();
            }
        }
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            return parse_f32(&unary.expr).map(|v| -v);
        }
        _ => {}
    }
    None
}

fn parse_i32(expr: &Expr) -> Option<i32> {
    if let Expr::Lit(lit) = expr
        && let Lit::Int(i) = &lit.lit
    {
        return i.base10_parse().ok();
    }
    None
}

fn parse_string(expr: &Expr) -> Option<String> {
    if let Expr::Lit(lit) = expr
        && let Lit::Str(s) = &lit.lit
    {
        return Some(s.value());
    }
    None
}

fn is_u32_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
    {
        return seg.ident == "u32";
    }
    false
}

fn snake_to_title_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                let upper: String = first.to_uppercase().collect();
                upper + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn snake_to_pascal_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                let upper: String = first.to_uppercase().collect();
                upper + chars.as_str()
            })
        })
        .collect::<String>()
}
