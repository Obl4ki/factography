use entities::recipes::Recipe;

#[derive(Debug, Clone, Default)]
pub struct Level {
    pub recipes: Vec<Recipe>,
}

impl Level {
    pub fn new(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }
}

impl From<Vec<Recipe>> for Level {
    fn from(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }
}
