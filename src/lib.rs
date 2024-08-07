#![allow(dead_code, unused)]

use proc_macro::TokenStream;

use syn::{DeriveInput, ItemTrait, Meta, parse_macro_input};
use syn::parse::Parser;
use syn::spanned::Spanned;

use crate::__derive_internal::kanu_derive_internal;
use crate::__trait_internal::kanu_trait_internal;

mod __trait_internal;
mod __derive_internal;
mod keywords;

#[cfg(test)]
mod tests {}


#[proc_macro_derive(kanu, attributes(kanu))]
pub fn kanu_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = kanu_derive_internal(input);

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn kanu_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(attr as Meta);
    let item = parse_macro_input!(item as ItemTrait);

    let output = kanu_trait_internal(meta, item);

    TokenStream::from(output)
}

