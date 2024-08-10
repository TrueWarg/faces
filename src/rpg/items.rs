use bevy::prelude::Component;

#[derive(Component, Clone)]
pub enum ConsumableItem {
    Dumplings(Dumplings),
    Venison(Venison),
}

#[derive(Component, Clone)]
pub struct Dumplings {
    pub health: i32,
    pub energy: i32,
}

#[derive(Component, Clone)]
pub struct Venison {
    pub health: i32,
    pub energy: i32,
}
