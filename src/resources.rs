use bevy::{prelude::{Handle, Resource}, sprite::TextureAtlas};

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}