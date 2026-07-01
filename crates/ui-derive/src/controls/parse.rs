use proc_macro2::Ident;
use syn::{DeriveInput, Expr, Lit, Type, punctuated::Punctuated, token::Comma};

pub fn parse_post_attr(input: &DeriveInput) -> Option<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("controls") {
            continue;
        }

        let nested: Punctuated<syn::Meta, Comma> =
            attr.parse_args_with(Punctuated::parse_terminated).ok()?;

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

pub fn parse_f32(expr: &Expr) -> Option<f32> {
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

pub fn parse_i32(expr: &Expr) -> Option<i32> {
    if let Expr::Lit(lit) = expr
        && let Lit::Int(i) = &lit.lit
    {
        return i.base10_parse().ok();
    }
    None
}

pub fn parse_string(expr: &Expr) -> Option<String> {
    if let Expr::Lit(lit) = expr
        && let Lit::Str(s) = &lit.lit
    {
        return Some(s.value());
    }
    None
}

pub fn is_u32_type(ty: &Type) -> bool {
    path_type_is(ty, "u32")
}

pub fn is_bool_type(ty: &Type) -> bool {
    path_type_is(ty, "bool")
}

fn path_type_is(ty: &Type, ident: &str) -> bool {
    if let Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
    {
        return seg.ident == ident;
    }
    false
}

pub fn variants<'a>(fields: impl Iterator<Item = &'a Ident>) -> Vec<Ident> {
    fields
        .map(|field| Ident::new(&snake_to_pascal_case(&field.to_string()), field.span()))
        .collect()
}

pub fn snake_to_title_case(name: &str) -> String {
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
