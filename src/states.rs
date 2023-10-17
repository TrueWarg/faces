use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Exporation,
    ProhibitedPlace,
    Fighting,
    Dialog,
    Inventory,
    Skills,
    Journal,
    CatScene,
    GameOver,
}
