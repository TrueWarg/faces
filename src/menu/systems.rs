use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::prelude::Component;
use bevy::prelude::Font;
use bevy::prelude::Image;
use bevy::prelude::Resource;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct ButtonActive(bool);

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {}
}
