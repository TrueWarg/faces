use bevy::{
    prelude::{AssetServer, Assets, Commands, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use crate::resources::CharsetAsset;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("terminal8x8_transparent.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CharsetAsset {
        atlas: texture_atlas_handle.clone(),
    });
}
