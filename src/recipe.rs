use crate::item::Item;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Recipes {
    IronOre,
    IronIngot,
    PureIronIngot,
    SolidSteelIngot,
    SteelBeam,
    SteelPipe,
}

impl TryFrom<String> for Recipes {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "pure iron ingot" => Ok(Self::PureIronIngot),
            "solid steel ingot" => Ok(Self::SolidSteelIngot),
            "steel beam" => Ok(Self::SteelBeam),
            "steel pipe" => Ok(Self::SteelPipe),
            _ => Err(format!("{value} is not recognized")),
        }
    }
}

impl Recipes {
    pub fn get_ingredients(&self) -> Vec<Item> {
        match self {
            Recipes::IronOre => vec![],
            Recipes::IronIngot => vec![Item::IronOre(30)],
            Recipes::PureIronIngot => vec![Item::IronOre(35), Item::Water(20)],
            Recipes::SolidSteelIngot => vec![Item::IronIngot(40), Item::Coal(40)],
            Recipes::SteelBeam => vec![Item::SteelIngot(60)],
            Recipes::SteelPipe => vec![Item::SteelIngot(30)],
        }
    }

    pub fn get_outputs(&self) -> Vec<Item> {
        match self {
            Recipes::IronOre => vec![],
            Recipes::IronIngot => vec![Item::IronIngot(30)],
            Recipes::PureIronIngot => vec![Item::IronIngot(65)],
            Recipes::SolidSteelIngot => vec![Item::SteelIngot(60)],
            Recipes::SteelBeam => vec![Item::SteelBeam(15)],
            Recipes::SteelPipe => vec![Item::SteelPipe(20)],
        }
    }
}



impl Item {
    pub fn get_recipes(&self) -> Vec<Recipes> {
        match self {
            Item::IronOre(_) => vec![],
            Item::Water(_) => todo!(),
            Item::IronIngot(_) => todo!(),
            Item::SteelIngot(_) => todo!(),
            Item::SteelBeam(_) => todo!(),
            Item::SteelPipe(_) => todo!(),
            Item::Coal(_) => todo!(),
        }
    }
}