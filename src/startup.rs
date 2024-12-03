use bevy::prelude::{Camera2dBundle, Commands, Vec3};

use crate::core::entities::MainCamera;

pub fn setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn((MainCamera, cam));
}
