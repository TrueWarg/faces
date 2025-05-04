use crate::rpg::TargetProps;
use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub enum ConsumableItem {
    Dumplings(Dumplings),
    Venison(Venison),
}

impl ConsumableItem {
    pub fn default_dumplings() -> Self {
        ConsumableItem::Dumplings(Dumplings {
            health: 20,
            energy: 5,
        })
    }

    pub fn default_venison() -> Self {
        ConsumableItem::Venison(Venison {
            health: 40,
            energy: 5,
        })
    }
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
