use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Exploration,
    ProhibitedPlace,
    Fighting,
    Dialog,
    InventoryAndAbilities,
    Journal,
    CatScene,
    GameOver,
    DevSetting,
    Character,
}
