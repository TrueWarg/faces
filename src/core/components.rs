use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Debug, Clone, Copy)]
pub struct BodyYOffset {
    pub value: f32,
}

impl BodyYOffset {
    pub fn create(value: f32) -> BodyYOffset {
        return BodyYOffset { value };
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LevelYMax {
    pub value: f32,
}

impl LevelYMax {
    pub fn create(value: f32) -> LevelYMax {
        return LevelYMax { value };
    }
}

#[derive(Component, Debug)]
pub struct Description {
    pub text: String,
}
