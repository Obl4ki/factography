use std::fmt::Debug;

use macro_lib::recipes;

use crate::item::*;

#[derive(Debug)]
pub struct Ingredient {
    pub qtty: u32,
    pub item: Box<dyn Item>,
}

pub trait Recipe: Debug {
    fn get_ingredients(&self) -> Vec<Ingredient>;
}

recipes! {
    IronIngot: 30 IronOre -> 30 IronIngot
    PureIronIngot: 35 IronOre 20 Water -> 30 IronIngot

    SteelIngot:
        40 IronIngot
        40 Coal
            -> 60 SteelIngot
    SteelBeam: 60 SteelIngot -> 15 SteelBeam
    SteelPipe: 30 SteelIngot -> 20 SteelPipe

}
