use crate::app::units::MinerUnit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Miner(MinerUnit),
}
