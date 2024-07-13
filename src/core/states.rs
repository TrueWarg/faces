use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
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
    #[default]
    DevSetting
}
