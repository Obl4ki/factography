use proc_macro::TokenStream;
mod craftable_items;
mod recipes;

#[proc_macro]
pub fn items(input: TokenStream) -> TokenStream {
    craftable_items::craftable_items_macro(input)
}

#[proc_macro]
pub fn recipes(input: TokenStream) -> TokenStream {
    recipes::recipes_macro(input)
}
