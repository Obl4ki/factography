use std::fmt::Debug;

use crate::item::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ingredient {
    pub qtty: u32,
    pub item: Item,
}

impl Ingredient {
    pub fn new(qtty: u32, item: Item) -> Self {
        Self { qtty, item }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recipe {
    IronIngot,
    PureIronIngot,
    SolidSteelIngot,
    SteelIngot,
    SteelBeam,
    SteelPipe,
}

impl Recipe {
    pub fn get_ingredients(&self) -> Vec<Ingredient> {
        match self {
            Recipe::PureIronIngot => {
                vec![
                    Ingredient::new(35, Item::Natural(ResourceItem::IronOre)),
                    Ingredient::new(20, Item::Natural(ResourceItem::Water)),
                ]
            }
            Recipe::SolidSteelIngot => vec![
                Ingredient::new(40, Item::Crafted(CraftableItem::IronIngot)),
                Ingredient::new(40, Item::Natural(ResourceItem::Coal)),
            ],
            Recipe::SteelIngot => vec![
                Ingredient::new(30, Item::Natural(ResourceItem::IronOre)),
                Ingredient::new(30, Item::Natural(ResourceItem::Coal)),
            ],
            Recipe::SteelBeam => vec![Ingredient::new(
                60,
                Item::Crafted(CraftableItem::SteelIngot),
            )],
            Recipe::SteelPipe => vec![Ingredient::new(
                30,
                Item::Crafted(CraftableItem::SteelIngot),
            )],
            Recipe::IronIngot => vec![Ingredient::new(30, Item::Natural(ResourceItem::IronOre))],
        }
    }

    pub fn get_output(&self) -> (u32, CraftableItem) {
        match self {
            Recipe::PureIronIngot => (65, CraftableItem::IronIngot),
            Recipe::SolidSteelIngot => (60, CraftableItem::SteelIngot),
            Recipe::SteelIngot => (30, CraftableItem::SteelIngot),
            Recipe::SteelBeam => (15, CraftableItem::SteelBeam),
            Recipe::SteelPipe => (20, CraftableItem::SteelPipe),
            Recipe::IronIngot => (30, CraftableItem::IronIngot),
        }
    }
}

impl Recipe {
    pub fn contains_natural_items(&self) -> bool {
        for ingredient in self.get_ingredients() {
            if let Item::Natural(_) = ingredient.item {
                return true;
            }
        }
        false
    }
}
