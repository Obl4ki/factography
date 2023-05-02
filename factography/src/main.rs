#![allow(unused_imports)]
mod item;
mod recipe;

use item::CraftableItem;
use recipe::Ingredient;
use recipe::Recipe;

use crate::item::Item;

fn main() -> Result<(), String> {
    let target_item = CraftableItem::SteelBeam;
    let recipe_paths = get_recipe_paths(target_item);

    for (path_idx, path) in recipe_paths.iter().enumerate() {
        println!("-------------");
        println!("Path {path_idx}:");
        for recipe in path {
            let ingredients = recipe.get_ingredients();
            println!("\tRecipe: {recipe:?}\tIngredients: {ingredients:?}")
        }
    }

    Ok(())
}

pub fn get_recipe_paths(item: CraftableItem) -> Vec<Vec<Recipe>> {
    let mut all_recipe_paths = vec![item.get_all_recipes()];
    let mut full_paths = vec![];

    while let Some(mut current_path) = all_recipe_paths.pop() {
        let recipe_to_unroll = current_path.pop().unwrap();
        let ingredients_for_recipe = recipe_to_unroll.get_ingredients();
        current_path.push(recipe_to_unroll);
        if recipe_to_unroll.contains_natural_items() {
            full_paths.push(current_path.clone());
            continue;
        }
        for ing in ingredients_for_recipe {
            if let Item::Crafted(item) = ing.item {
                let child_recipes = item.get_all_recipes();
                for child_recipe in child_recipes {
                    let mut new_path = current_path.clone();
                    new_path.push(child_recipe);
                    all_recipe_paths.push(new_path);
                }
            }
        }
    }

    full_paths
}
