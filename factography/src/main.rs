use entities::item::CraftableItem;

use crate::algo::get_recipe_paths;

mod algo;
mod level;
mod path;
mod paths;

fn main() -> Result<(), String> {
    let target_item = CraftableItem::Gizmo;
    let recipe_paths = get_recipe_paths(target_item);
    println!("\n\n{recipe_paths:#?}");

    Ok(())
}
