mod base;
mod mine;
mod mine_construction;
mod ore;
mod stone;
mod structure_trait;
mod structures_enum;
mod tunnel;

pub use base::BaseBuilding;
pub use mine::Mine;
pub use mine_construction::MineConstruction;
pub use ore::Ore;
pub use stone::Stone;
pub use structure_trait::Structure;
pub use structures_enum::Structures;
pub use tunnel::Tunnel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingTerrain {
  Stone,
  Ore,
}
