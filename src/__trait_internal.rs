use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Expr, ItemTrait, Lit, Meta};
use syn::parse::Parser;

pub(crate) fn kanu_trait_internal(meta: Meta, item: ItemTrait) -> proc_macro2::TokenStream {
    let mut target_name = String::new();

    if let Meta::NameValue(meta) = meta {
        match meta.value {
            Expr::Lit(expr) => {
                if let Lit::Str(lit_str) = &expr.lit {
                    target_name = lit_str.value()
                }
            }
            _ => {
                panic!("")
            }
        }
    };

    if target_name.is_empty() {
        panic!("target name cannot be empty")
    }

    let target = Ident::new(target_name.as_str(), Span::call_site());

    for attr in item.attrs {
        println!("{}", attr.meta.path().get_ident().unwrap())
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