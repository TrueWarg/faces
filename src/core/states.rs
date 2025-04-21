use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Exploration,
    Fighting,
    Dialog,
    InventoryAndAbilities,
    CatScene,
    GameOver,
    DevSetting,
    Character,
}
