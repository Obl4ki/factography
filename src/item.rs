use crate::recipe::Recipes;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    IronOre(u32),
    Water(u32),
    IronIngot(u32),
    SteelIngot(u32),
    SteelBeam(u32),
    SteelPipe(u32),
    Coal(u32),
}
