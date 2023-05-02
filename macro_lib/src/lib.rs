#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

mod items;
mod recipes;

#[proc_macro]
pub fn items(input: TokenStream) -> TokenStream {
    items::items_macro(input)
}

#[proc_macro]
pub fn recipes(input: TokenStream) -> TokenStream {
    recipes::recipes_macro(input)
}
