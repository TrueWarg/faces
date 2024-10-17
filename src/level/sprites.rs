use bevy::{asset::Handle, ecs::system::Resource, render::texture::Image};

#[derive(Resource)]
pub struct WoodenChestSprites {
    pub closed: Handle<Image>,
    pub full: Handle<Image>,
    pub empty: Handle<Image>,
}
