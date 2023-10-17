#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
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
