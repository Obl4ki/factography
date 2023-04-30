use std::fmt::Debug;

use crate::recipe as r;
use macro_lib::items;
use r::Recipe;

pub trait Item: Debug {
    fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>>;
}

items! {
    IronOre: _;
    Water: _;
    Coal: _;
    IronIngot: IronIngot PureIronIngot;
    SteelIngot: SteelIngot;
    SteelBeam: SteelBeam;
    SteelPipe: SteelPipe;
}

// #[derive(Debug)]
// pub struct IronOre;
// impl Item for IronOre {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![]
//     }
// }

// #[derive(Debug)]
// pub struct Water;
// impl Item for Water {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![]
//     }
// }

// #[derive(Debug)]
// pub struct IronIngot;
// impl Item for IronIngot {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![Box::new(r::PureIronIngot)]
//     }
// }

// #[derive(Debug)]
// pub struct Coal;
// impl Item for Coal {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![]
//     }
// }

// #[derive(Debug)]
// pub struct SteelIngot;
// impl Item for SteelIngot {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![Box::new(r::SolidSteelIngot)]
//     }
// }

// #[derive(Debug)]
// pub struct SteelBeam;
// impl Item for SteelBeam {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![Box::new(r::SteelBeam)]
//     }
// }

// #[derive(Debug)]
// pub struct SteelPipe;
// impl Item for SteelPipe {
//     fn get_all_recipes(&self) -> Vec<Box<dyn Recipe>> {
//         vec![Box::new(r::SteelPipe)]
//     }
// }
