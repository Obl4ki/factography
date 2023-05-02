use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;

use syn::{parse_macro_input, Ident, Token};

#[derive(Debug)]
struct Items {
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: Ident,
    recipes: Vec<Ident>,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = vec![];
        while !input.is_empty() {
            let item_name: Ident = input.parse()?;
            let mut recipe_names: Vec<Ident> = vec![];
            let _: Colon = input.parse()?;

            if input.peek(Token![_]) {
                let _: Token![_] = input.parse()?;
                items.push(Item {
                    name: item_name,
                    recipes: vec![],
                });
                let _: Token![;] = input.parse()?;
                continue;
            }

            while !input.peek(Token![;]) {
                let recipe_name = input.parse()?;
                recipe_names.push(recipe_name);
            }
            let _: Token![;] = input.parse()?;
            items.push(Item {
                name: item_name,
                recipes: recipe_names,
            })
        }

        Ok(Items { items })
    }
}

pub fn items_macro(input: TokenStream) -> TokenStream {
    let Items { items } = parse_macro_input!(input as Items);

    let mut ast = quote!();
    for item in items {
        let Item { name, recipes } = item;

        let recipes = recipes.into_iter().fold(quote!(), |ast, ing| {
            quote!(
                #ast
                Box::new(crate::recipe::#ing),
            )
        });

        ast = quote!(
            #ast

            #[derive(Debug)]
            pub struct #name;
            impl Item for #name {
                fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
                    vec![#recipes]
                }
            }
        );
    }

    ast.into()
}
