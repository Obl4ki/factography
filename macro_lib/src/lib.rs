#![feature(proc_macro_diagnostic)]
use std::vec;

use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::LitInt;
use syn::{parse_macro_input, Ident, Token};

#[derive(Debug)]

struct Recipes {
    recipes: Vec<Recipe>,
}

#[derive(Debug)]

struct Recipe {
    name: Ident,
    output: Ingredient,
    ingredients: Vec<Ingredient>,
}

#[derive(Debug)]

struct Ingredient {
    qtty: u32,
    item: Ident,
}

impl Parse for Recipes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut recipes = vec![];

        while !input.is_empty() {
            let recipe_name: Ident = input.parse()?;
            let _: Colon = input.parse()?;
            let mut ingredients: Vec<Ingredient> = vec![];
            while !input.peek(Token![->]) {
                let qtty: LitInt = input.parse()?;
                let ing_name = input.parse()?;
                let _: Option<Token![,]> = input.parse()?;
                ingredients.push(Ingredient {
                    qtty: qtty.base10_parse()?,
                    item: ing_name,
                });
            }

            let _: Token![->] = input.parse()?;
            let out_qtty: LitInt = input.parse()?;
            let out_name: Ident = input.parse()?;
            recipes.push(Recipe {
                name: recipe_name,

                output: Ingredient {
                    qtty: out_qtty.base10_parse()?,
                    item: out_name,
                },
                ingredients,
            })
        }

        Ok(Recipes { recipes })
    }
}

#[proc_macro]
pub fn recipes(input: TokenStream) -> TokenStream {
    let Recipes { recipes } = parse_macro_input!(input as Recipes);

    let mut ast = quote!();
    for recipe in recipes {
        let Recipe {
            name: recipe_name,
            output,
            ingredients,
        } = recipe;

        let ingredients = ingredients.into_iter().fold(quote!(), |ast, ing| {
            let ingredient_quantity = ing.qtty;
            let item = ing.item;

            quote!(
                #ast
                Ingredient {
                    qtty: #ingredient_quantity,
                    item: Box::new(crate::item::#item)
                },
            )
        });

        ast = quote!(
            #ast

            #[derive(Debug)]
            pub struct #recipe_name;
            impl Recipe for #recipe_name {
                fn get_ingredients(&self) -> Vec<Ingredient> {
                    vec![#ingredients]
                }
            }
        );
    }

    ast.into()
}

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

#[proc_macro]
pub fn items(input: TokenStream) -> TokenStream {
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
