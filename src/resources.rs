use bevy::{prelude::{Handle, Resource}, sprite::TextureAtlas};

#[derive(Resource)]
pub struct AssetsPack {
    pub atlas: Handle<TextureAtlas>,
}