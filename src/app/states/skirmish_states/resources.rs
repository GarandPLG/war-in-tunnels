use crate::app::threads::TICK_MS;

pub const WOOD_DELIVERY_AMOUNT: u32 = 5;
pub const WOOD_DELIVERY_INTERVAL_MS: u64 = 5000;
pub const WOOD_DELIVERY_TICKS: u64 = WOOD_DELIVERY_INTERVAL_MS.div_ceil(TICK_MS as u64);
pub const MINE_PRODUCTION_TICKS: u64 = 1000_u64.div_ceil(TICK_MS as u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resources {
    pub wood: u32,
    pub metal: u32,
}

impl Resources {
    pub fn can_afford(&self, cost: Self) -> bool {
        self.wood >= cost.wood && self.metal >= cost.metal
    }

    pub fn spend(&mut self, cost: Self) -> bool {
        if !self.can_afford(cost) {
            return false;
        }
        self.wood -= cost.wood;
        self.metal -= cost.metal;
        true
    }
    pub fn new(wood: u16, metal: u16) -> Self {
        Self {
            wood: u32::from(wood),
            metal: u32::from(metal),
        }
    }

    pub fn add_wood(&mut self, amount: u32) -> () {
        self.wood = self.wood.saturating_add(amount);
    }

    pub fn add_metal(&mut self, amount: u32) -> () {
        self.metal = self.metal.saturating_add(amount);
    }
}
