use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Expr, ItemTrait, Lit, Meta, TraitItem, TraitItemFn};
use crate::__internal::validations::validate_target_name;
use crate::keywords::KeyWord;

pub(crate) fn kanu_trait_internal(meta: Meta, items: ItemTrait) -> TokenStream {
    let target_name = get_target_name(meta);

    validate_target_name(&target_name);

    let target = Ident::new(target_name.as_str(), Span::call_site());

    let ident = items.ident;

    let mut fns = vec![];

    for item in items.items {
        match item {
            TraitItem::Fn(trait_item_fn) => {
                fns.push(parse_fn(trait_item_fn))
            }
            _ => {}
        }
    }

    let expanded = quote! {
        #[allow(non_snake_case)]
        impl #target {
            #(#fns)*
        }
    };

    expanded
}

fn get_target_name(meta: Meta) -> String {
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

    target_name
}

fn parse_fn(trait_item_fn: TraitItemFn) {
    let fn_ident = trait_item_fn.sig.ident;
    let keywords = KeyWord::split_to_keywords(&fn_ident, trait_item_fn.sig.inputs.len());


}