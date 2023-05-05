use crate::level::Level;
use crate::path::Path;
use crate::paths::Paths;
use entities::item::{CraftableItem, Item};

pub fn branch_paths_by_item(paths: Paths, item: CraftableItem) -> Paths {
    let mut new_paths = vec![];
    let variants = item.get_all_recipes();

    for variant in &variants {
        for path in &paths.paths {
            let mut new_path = path.clone();
            new_path
                .levels
                .last_mut()
                .unwrap()
                .recipes
                .push(variant.to_owned());

            new_paths.push(new_path);
        }
    }

    Paths::new(new_paths)
}

pub fn unroll_path(path: Path) -> Paths {
    let last_level = path.levels.last().unwrap().to_owned();

    let mut result_paths = vec![path];

    if last_level.recipes.is_empty() {
        return Paths {
            paths: result_paths,
        };
    }

    for path in &mut result_paths {
        path.levels.push(Level::default());
    }

    for recipe in last_level.recipes {
        let ingredients = recipe.get_ingredients();
        for ing in ingredients {
            result_paths = if let Item::Crafted(item) = ing.item {
                branch_paths_by_item(result_paths.into(), item).paths
            } else {
                result_paths
            }
        }
    }

    Paths::new(result_paths)
}

pub fn get_recipe_paths(item: CraftableItem) -> Paths {
    let mut recipe_paths_queue = vec![Path::new(vec![Level::new(item.get_all_recipes())])];
    let mut terminal_paths = vec![];
    while let Some(current_path) = recipe_paths_queue.pop() {
        if current_path.is_terminal() {
            terminal_paths.push(current_path);
            continue;
        }

        let new_paths = unroll_path(current_path);
        recipe_paths_queue.extend(new_paths.paths);
    }

    Paths::new(terminal_paths)
}
