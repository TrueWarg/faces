use bevy::{
    prelude::{AssetServer, Assets, Camera2dBundle, Commands, Component, Res, ResMut, Vec3},
    sprite::TextureAtlas,
};

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
}

#[derive(Component)]
pub struct MainCamera;
