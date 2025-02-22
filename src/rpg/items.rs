use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub enum ConsumableItem {
    Dumplings(Dumplings),
    Venison(Venison),
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
