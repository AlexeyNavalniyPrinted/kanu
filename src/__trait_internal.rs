use std::any::Any;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Expr, ItemTrait, Lit, LitStr, Meta, Pat, TraitItem, TraitItemFn};
use syn::parse::Parser;
use crate::keywords::KeyWord;

pub(crate) fn kanu_trait_internal(meta: Meta, items: ItemTrait) -> TokenStream {
    let mut target_name = String::new();

    if let Meta::NameValue(meta) = meta {
        match meta.value {
            Expr::Lit(expr) => {
                if let Lit::Str(lit_str) = &expr.lit {
                    target_name = lit_str.value()
                }
            }
            _ => {
                panic!("Expression type is not \"Literal\"")
            }
        }
    };

    check_if_target_name_correct(&target_name);

    let target = Ident::new(target_name.as_str(), Span::call_site());

    let ident = items.ident;

    let mut fns = quote! {};

    for item in items.items {
        match item {
            TraitItem::Fn(trait_item_fn) => {
                fns.extend(parse_fn(trait_item_fn))
            }
            _ => {}
        }
    }

    let expanded = quote! {
        impl #target {
            fn hello() {
                println!("Works")
            }
        }
    };

    expanded
}

fn check_if_target_name_correct(target_name: &String) {
    if target_name.is_empty() {
        panic!("Target name cannot be empty")
    }
    if !target_name.is_ascii() {
        panic!("Target name should contain only letters (Non ascii characters not allowed)")
    }

    for char in target_name.chars() {
        if char.is_ascii_digit() {
            panic!("Target name should contain only letters (No numbers allowed)")
        }

    }
}

fn parse_fn(trait_item_fn: TraitItemFn) -> TokenStream {
    let fn_ident = trait_item_fn.sig.ident.to_string();
    let keywords = KeyWord::split_to_keywords(&fn_ident, trait_item_fn.sig.inputs.len());

    println!("{:?}", keywords);

    let expanded = quote! {

    };

    expanded
}
