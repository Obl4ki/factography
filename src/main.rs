use crate::{item::Item, recipe::Recipes};

mod item;
mod recipe;

fn main() -> Result<(), String> {
    let recipe_name = "Steel pipe".to_string();
    let target_recipe: Recipes = Recipes::try_from(recipe_name)?;
    let start_resources: Vec<Item> = vec![Item::Coal(270), Item::Water(300), Item::IronOre(270)];

    let mut q = vec![target_recipe.clone()];

    while let Some(rec) = q.pop() {
        q.extend(
            rec.get_ingredients()
                .into_iter()
                .flat_map(|item| item.get_recipes()),
        );
        println!("{rec:?}");
    }

    // let println!("{:#?}", recipe.get_ingredients());
    Ok(())
}
