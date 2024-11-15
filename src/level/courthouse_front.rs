use bevy::app::Plugin;
use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::SpriteBundle;
use bevy::prelude::States;
use bevy::prelude::TextureAtlasLayout;
use bevy::prelude::Transform;
use bevy_rapier2d::dynamics::RigidBody;
use crate::core::states::GameState;
use crate::core::z_index::FLOOR_Z;

pub struct CourtHouseFrontPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for CourtHouseFrontPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnExit(GameState::Exploration), unload);
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_ground(&mut commands, &asset_server);
}

fn spawn_ground(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/ground.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, FLOOR_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn unload() {}