use bevy::{
    prelude::{Handle, Resource},
    sprite::TextureAtlasLayout,
};

#[derive(Resource)]
pub struct AssetsPack {
    pub atlas: Handle<TextureAtlasLayout>,
}
