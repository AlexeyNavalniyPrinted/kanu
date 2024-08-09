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


fn parse_fn(trait_item_fn: TraitItemFn) -> proc_macro2::TokenStream {
    let fn_ident = trait_item_fn.sig.ident;
    let keywords = KeyWord::split_to_keywords(&fn_ident, trait_item_fn.sig.inputs.len());

    if let FnArg::Typed(_) = trait_item_fn.sig.inputs[0] {
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

        let arg_names: Vec<Ident> = args.iter().map(|arg| arg.arg_name.clone()).collect();

        let arg_names: Vec<proc_macro2::TokenStream> = args.iter().map(|arg| {
            let arg_ident = &arg.arg_name;
            quote! {#arg_ident}
        }).collect();

        let arg_types: Vec<Type> = args.iter().map(|arg| arg.arg_type.clone()).collect();


        let keywords: Vec<proc_macro2::TokenStream> = match keywords[0] {
            KeyWord::Set => {
                let mut by_found = false;
                let mut vector = vec![];

                for kw in keywords {
                    if kw == KeyWord::By {
                        by_found = true;
                    }

                    let kw_string = kw.to_string();

                    if !(kw == KeyWord::And && !by_found) {
                        vector.push(quote! {#kw_string})
                    }
                }
                vector
            }
            _ => {
                keywords.iter().map(|kw| {
                    let kw_string = kw.to_string();
                    quote! { #kw_string }
                }).collect()
            }
        };


        let output_type = trait_item_fn.sig.output.to_token_stream();

        let expanded = quote! {
            pub fn #fn_ident(#(#arg_names: #arg_types),*) #output_type {
                // New string
                let mut sql = String::new();

                // Ident vector
                let arg_names = vec![#(#arg_names),*];

                // Current arg
                let mut current_arg_index = 0;

                // Keywords vector
                let keywords = vec![#(#keywords),*];

                for (index, keyword) in keywords.iter().enumerate() {
                    println!("index: {} ; sql: {}", index, sql);
                    // Push keywords
                    sql.push_str(keyword);

                    if index == 0 {
                        // Push table name
                        sql.push_str(Self::table());

                        // Push method specific stuff
                        match keywords[0] {
                            "insert into " => { sql.push_str(" values( ") },
                            "update " => { sql.push_str(" set ") }
                            _ => { sql.push_str(" where "); }
                        }
                    }

                    match keyword {
                        // Checks whether keyword is not an argument
                        &"select * from " | &"delete from "  | &"and " | &"or " | &"insert into " | &"update " | &" where " => {}
                        _ =>  {
                            if current_arg_index != arg_names.len() - 1 {
                                // increase current arg index
                                current_arg_index += 1;
                            }

                            // Push arg
                            sql.push_str(&format!(" '{}' ", arg_names[current_arg_index]));
                        }
                    }

                    if current_arg_index != arg_names.len() - 1 {
                        // increase current arg index
                        current_arg_index += 1;
                    }

                    // Push arg
                    sql.push_str(&format!(" '{}' ", arg_names[current_arg_index]));

                    // Correctly end sql statement
                    if index == keywords.len() - 1 {
                        if keywords[0] == "insert into " {
                            // Insert into specific
                            for arg_name in &arg_names {
                                sql.push_str(&format!(" '{}' ", arg_name));
                                if arg_name != arg_names.last().unwrap() {
                                    sql.push_str(", ");
                                }
                            }
                            sql.push_str(")")
                        }

                        sql.push_str(";")
                    }
                }
                return sql
            }
        };

        return expanded;
    }

    quote! {}
}
