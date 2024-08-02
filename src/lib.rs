#![allow(dead_code, unused)]

use proc_macro::{TokenStream};
use proc_macro2::{Ident,Span};
use quote::quote;
use syn::{DeriveInput, Expr, Fields, ItemTrait, Lit, LitStr, Meta, MetaNameValue, parse_macro_input, Token};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

#[cfg(test)]
mod tests {}


#[proc_macro_derive(kanu, attributes(kanu))]
pub fn kanu_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let data = input.data;

    let mut table = String::new();
    let mut migrate = false;

    for attr in &input.attrs {
        if !attr.path().is_ident("kanu") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("migrate") {
                migrate = true;
                return Ok(());
            }

            if meta.path.is_ident("table") {
                let value = meta.value().unwrap();
                table = value.parse::<LitStr>().unwrap().value();
                return Ok(());
            }
            Err(meta.error("unrecognized repr"))
        });
    }

    if let syn::Data::Struct(data) = data {
        match &data.fields {
            Fields::Named(named) => for field in named.named.clone() {
                for attr in field.attrs {
                    if attr.path().is_ident("kanu") {
                        attr.parse_nested_meta(|meta| {
                            Ok(())
                        });
                    }
                }
            },
            _ => {}
        }
    }

    let expanded = quote! {
        impl #name {

        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn kanu_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Meta);

    let mut target_name = String::new();

    if let Meta::NameValue(meta) = attr {
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
    
    let input = parse_macro_input!(item as ItemTrait);

    for attr in input.attrs {
        println!("{}", attr.meta.path().get_ident().unwrap())
    }

    let expanded = quote! {
        impl #target {
            fn hello() {
                println!("Works")
            }
        }
    };

    TokenStream::from(expanded)
}

