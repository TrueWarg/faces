use bevy::prelude::Component;
use crate::rpg::TargetProps;

#[derive(Component, Clone, Debug)]
pub enum ConsumableItem {
    Dumplings(Dumplings),
    Venison(Venison),
}

impl ConsumableItem {
    pub fn apply(&self, target: &mut TargetProps) {
        match self {
            ConsumableItem::Dumplings(value) => {
                target.health.increase(value.health);
                target.energy.increase(value.energy);
            }
            ConsumableItem::Venison(value) => {
                target.health.increase(value.health);
                target.energy.decrease(value.energy);
            }
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Dumplings {
    pub health: i32,
    pub energy: i32,
}

#[derive(Component, Clone, Debug)]
pub struct Venison {
    pub health: i32,
    pub energy: i32,
}
