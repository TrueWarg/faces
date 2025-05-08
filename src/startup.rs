use crate::core::entities::MainCamera;
use bevy::prelude::{
    default, Camera, Camera2dBundle, ClearColorConfig, Commands, IsDefaultUiCamera, Vec3,
};
use bevy::render::view::RenderLayers;

pub fn setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    // cam.camera = Camera {
    //     order: 1,
    //     clear_color: ClearColorConfig::None,
    //     ..default()
    // };
    commands.spawn((IsDefaultUiCamera, MainCamera, cam));
    //
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    cam.camera = Camera {
        order: 2,
        clear_color: ClearColorConfig::None,
        ..default()
    };
    commands.spawn((RenderLayers::layer(2), cam));
}
