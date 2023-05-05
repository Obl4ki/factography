use entities::item::Item;

use crate::level::Level;

#[derive(Debug, Clone, Default)]
pub struct Path {
    pub levels: Vec<Level>,
}

impl Path {
    pub fn new(levels: Vec<Level>) -> Self {
        Self { levels }
    }
    pub fn is_terminal(&self) -> bool {
        self.levels.last().unwrap().recipes.iter().all(|rec| {
            rec.get_ingredients()
                .iter()
                .all(|ing| matches!(ing.item, Item::Natural(_)))
        })
    }
}

impl From<Vec<Level>> for Path {
    fn from(levels: Vec<Level>) -> Self {
        Self { levels }
    }
}
