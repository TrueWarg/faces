mod components;
mod entities;

use bevy::app::{App, Plugin};
use sickle_ui::SickleUiPlugin;
pub use components::*;
pub use entities::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin);
    }
}
