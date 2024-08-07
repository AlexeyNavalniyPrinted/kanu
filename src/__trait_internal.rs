use std::any::Any;
use std::collections::HashMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Expr, FnArg, ItemTrait, Lit, LitStr, Meta, Pat, TraitItem, TraitItemFn, Type};
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

fn check_if_target_name_correct(target_name: &String) {
    if target_name.is_empty() {
        panic!("Target name cannot be empty")
    }
    if !target_name.is_ascii() {
        panic!("Target name should contain only letters (Non ascii characters not allowed)")
    }

    for char in target_name.chars() {
        if char.is_ascii_digit() {
            panic!("Target name should contain only letters (No numbers are allowed)")
        }
        match char {
            '!' | '\"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/'
            | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|'
            | '}' | '~' | '\t' | '\n' | '\r' | '\x00'..='\x1F' | '\x7F' | ' ' => {
                panic!("Target name should contain only letters (No special characters are allowed)")
            }
            _ => {}
        }
    }
}

struct Arg {
    arg_name: Ident,
    arg_type: Type,
}


fn parse_fn(trait_item_fn: TraitItemFn) -> TokenStream {
    let fn_ident = trait_item_fn.sig.ident;
    let keywords = KeyWord::split_to_keywords(&fn_ident, trait_item_fn.sig.inputs.len());

    let mut args = vec![];

    for fn_arg in trait_item_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = fn_arg {
            let arg_type = *pat_type.ty;
            if let Pat::Ident(pat_ident) = *pat_type.pat {
                args.push(Arg {
                    arg_name: pat_ident.ident,
                    arg_type,
                });
            }
        }
    }

    let table_name = "tablename";

    let arg_names: Vec<Ident> = args.iter().map(|arg| arg.arg_name.clone()).collect();

    let arg_names: Vec<proc_macro2::TokenStream> = args.iter().map(|arg| { let arg_ident = &arg.arg_name ; quote! {#arg_ident}}).collect();

    let arg_types: Vec<Type> = args.iter().map(|arg| arg.arg_type.clone()).collect();
    let keywords: Vec<proc_macro2::TokenStream> = keywords.iter().map(|kw| {
        let kw_string = kw.to_string();
        quote! { #kw_string }
    }).collect();

    let output_type = trait_item_fn.sig.output.to_token_stream();

    let expanded = quote! {
        fn #fn_ident(#(#arg_names: #arg_types),*) #output_type {
            let mut sql = String::new();

            let arg_names = vec![#(#arg_names),*];
            let mut current_arg_index = 0;

            let keywords = vec![#(#keywords),*];

            for (index, keyword) in keywords.iter().enumerate() {
                sql.push_str(keyword);
                if index == 0 {
                    sql.push_str(#table_name);
                    match keywords[0] {
                        _ => { sql.push_str(" where "); }
                    }
                    continue;
                }

                if keywords[0] != "update " {
                    match keyword {
                        &"select * from " | &"update " | &"delete from " | &"insert into " | &"and " | &"or " => {continue}
                        _ =>  {}
                    }

                    if current_arg_index != arg_names.len() - 1 {
                        current_arg_index += 1;
                    }
                    sql.push_str(&format!(" '{}' ", arg_names[current_arg_index]));
                }

                if index == keywords.len() - 1 {
                    sql.push_str(";")
                }
            }
            return sql
        }
    };

    expanded
}
