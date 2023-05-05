use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::LitInt;
use syn::{parse_macro_input, Ident, Token};

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

pub fn recipes_macro(input: TokenStream) -> TokenStream {
    let Recipes { recipes } = parse_macro_input!(input as Recipes);

    let mut enum_variants_ast = quote!();
    let mut match_ingredients_ast = quote!();
    let mut match_output_ast = quote!();

    for recipe in &recipes {
        let Recipe {
            name: recipe_name,
            output,
            ingredients,
        } = recipe;

        enum_variants_ast = quote!(
            #enum_variants_ast
            #recipe_name,
        );
        // Recipe::PureIronIngot => {
        //     vec![
        //         Ingredient::new(35, Item::Natural(ResourceItem::IronOre)),
        //         Ingredient::new(20, Item::Natural(ResourceItem::Water)),
        //     ]
        // }
        let vec_of_ingredients_ast = quote!();
        ingredients.iter().fold(quote!(), |ast, ing| {
            let Ingredient { qtty, item } = ing;
            let ast = quote!(
                #ast
                // Ingredient::new(35, Item::)
            );
            ast
        });

        match_ingredients_ast = quote!(
            #match_ingredients_ast
            Recipe::#recipe_name
        )
    }

    let full_ast = quote!(
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Recipe {
            #enum_variants_ast
        }

        impl Recipe {
            pub fn get_ingredients(&self) -> Vec<Ingredient> {
                match self {
                    #match_ingredients_ast
                }
            }

            pub fn get_output(&self) -> (u32, CraftableItem) {
                match self {
                    #match_output_ast
                }
            }
        }

    );

    // ast.into()
    todo!()
}
