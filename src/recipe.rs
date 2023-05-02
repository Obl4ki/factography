use macro_lib::recipes;
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

    pub fn contains_natural_items(&self) -> bool {
        for ingredient in self.get_ingredients() {
            if let Item::Natural(_) = ingredient.item {
                return true;
            }
        }
        false
    }
}

// CraftableItem::SteelIngot => {
//     vec![I::new(40, CItem::IronIngot), I::new(40, CItem::Coal)]
// }
// CraftableItem::SteelPipe => vec![I::new(30, CItem::SteelIngot)],
// CraftableItem::SteelBeam => vec![I::new(60, CItem::SteelIngot)],
// CraftableItem::IronIngot => vec![I::new(35, CItem::IronO)],

// recipes! {
//     IronIngot: 30 IronOre -> 30 IronIngot
//     PureIronIngot: 35 IronOre 20 Water -> 30 IronIngot
//     IronAlloyIngot: 20 IronOre 20 CopperOre -> 50 IronIngot

//     SteelIngot:
//         40 IronIngot
//         40 Coal
//             -> 60 SteelIngot

//     SteelBeam: 60 SteelIngot -> 15 SteelBeam
//     SteelPipe: 30 SteelIngot -> 20 SteelPipe
// }
