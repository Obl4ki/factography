use colored::*;

use crate::algo::{get_recipe_paths, recipes_into_ingredients};
use entities::{
    item::{CraftableItem, Item},
    recipes::{Ingredient, Recipe},
};

mod algo;
mod level;
mod path;
mod paths;

fn main() -> Result<(), String> {
    let target_item = CraftableItem::SteelBeam;
    let recipe_paths = get_recipe_paths(target_item);

    let ingredients: Vec<Vec<Vec<Ingredient>>> = recipe_paths
        .paths
        .into_iter()
        .map(recipes_into_ingredients)
        .collect();

    let indent_increment = '\t';
    for (idx, path) in ingredients.iter().enumerate() {
        println!(
            "Option {}: {}",
            (idx + 1).to_string().green(),
            "Ingredient tree:".blue().bold()
        );

        let mut indent = String::new();
        for level in path {
            println!("{}{}", indent, "└──".blue().bold());
            for ingredient in level {
                println!(
                    "{}{}",
                    indent,
                    format!(
                        "{}└── {} {:?}",
                        indent_increment, ingredient.qtty, ingredient.item
                    )
                    .yellow()
                );
            }
            indent.push(indent_increment);
        }
        println!("{}", "------------".black());
    }

    Ok(())
}
