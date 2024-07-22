use bevy::prelude::Component;

#[derive(Component)]
pub enum Ability {
    NeckTwist {
        damage: i32,
    }
}
