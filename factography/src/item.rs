use std::fmt::Debug;

use crate::recipe::{Ingredient, Recipe};
use macro_lib::items;

// items! {
//     IronOre: _;
//     Water: _;
//     Coal: _;
//     CopperOre: _;
//     IronIngot: IronIngot PureIronIngot;
//     SteelIngot: SteelIngot;
//     SteelBeam: SteelBeam;
//     SteelPipe: SteelPipe;
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Item {
    Crafted(CraftableItem),
    Natural(ResourceItem),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceItem {
    IronOre,
    CopperOre,
    Water,
    Coal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CraftableItem {
    IronIngot,
    SteelIngot,
    SteelPipe,
    SteelBeam,
}

impl CraftableItem {
    pub fn get_all_recipes(&self) -> Vec<Recipe> {
        match self {
            CraftableItem::IronIngot => vec![Recipe::PureIronIngot, Recipe::IronIngot],
            CraftableItem::SteelIngot => vec![Recipe::SteelIngot, Recipe::SolidSteelIngot],
            CraftableItem::SteelPipe => vec![Recipe::SteelPipe],
            CraftableItem::SteelBeam => vec![Recipe::SteelBeam],
        }
    }
}

// impl CraftableItem {

// }
