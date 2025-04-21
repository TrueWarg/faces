use bevy::app::{App, Update};
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::prelude::{Changed, Commands, Interaction, Plugin, Query, Res};

mod resources;

pub use resources::*;
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ButtonSounds>()
            .init_resource::<Soundtrack>()
            .add_systems(Update, menu_button_hover_sounds_handle);
    }
}

fn menu_button_hover_sounds_handle(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, Changed<Interaction>>,
    audio_res: Res<ButtonSounds>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Hovered {
            commands.spawn(AudioBundle {
                source: audio_res.hover.clone(),
                settings: PlaybackSettings::ONCE,
            });
        }
    }
}
