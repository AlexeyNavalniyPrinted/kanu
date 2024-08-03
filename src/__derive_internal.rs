use std::collections::{HashMap, HashSet};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, LitStr};

pub(crate) fn kanu_derive_internal(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let attrs = parse_attrs(&input.attrs);
    let fields = parse_fields(input.data);

    let expanded = quote! {
        impl #name {

        }
    };

    expanded
}

#[cfg(feature = "migrations")]
struct Attrs {
    migrate: bool,
    table: String
}

#[cfg(not(feature = "migrations"))]
struct Attrs {
    table: String,
}

fn parse_attrs(attrs: &Vec<Attribute>) -> Attrs {
    let mut table = String::new();
    #[cfg(feature = "migrations")]
    let mut migrate = false;

    for attr in attrs {
        if !attr.path().is_ident("kanu") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            #[cfg(feature = "migrations")]
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
    };

    #[cfg(feature = "migrations")]
    return Attrs {
        table,
        migrate
    };
    #[cfg(not(feature = "migrations"))]
    return Attrs {
        table
    }
}

struct Column {
    sql_name: String,
    column_type: String // Change to enum
}

fn parse_fields(data: Data) -> Vec<Column> {
    let mut field_sql_names: Vec<Column> = vec![];

    let Data::Struct(data) = data else {
        panic!("Only structs allowed")
    };

    let fields = match data.fields {
        Fields::Named(named) => named,
        _ => {
            panic!("Unnamed or union fields are not allowed")
        }
    };

    for field in fields.named {
        let mut sql_name = String::new();
        let ident = field.ident.unwrap().to_string();
        let mut skip = false;
        for attr in field.attrs {
            if !attr.meta.path().is_ident("kanu") {
                continue
            }
            attr.parse_nested_meta(|meta| {
                if skip {
                    return Ok(());
                }

                if meta.path.is_ident("column") {
                    let Ok(value) = meta.value() else {
                        sql_name = ident.to_string();
                        return Ok(())
                    };

                    sql_name = value.parse::<LitStr>().unwrap().value();

                    return Ok(());
                }

                if meta.path.is_ident("skip") {
                    skip = true;
                    return Ok(());
                }

                Err(meta.error("unrecognized repr"))
            });
        }

        if !skip {
            field_sql_names.push(Column {
                sql_name,
                column_type: "".to_string(), // write
            })
        }
    };

    field_sql_names
}

#[cfg(feature = "migrations")]
fn migrate() -> TokenStream {
    let expanded = quote! {};

    expanded
}

