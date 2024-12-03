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
use bevy_rapier2d::geometry::Collider;
use crate::core::entities::LevelYMax;
use crate::core::states::GameState;
use crate::core::z_index::{calculate_z, FLOOR_Z, WALL_Z};

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
    let y_max = LevelYMax::create(500.0);

    spawn_ground(&mut commands, &asset_server);
    spawn_court_house(&mut commands, &asset_server, y_max);
    spawn_houses(&mut commands, &asset_server);
    spawn_right_forest(&mut commands, &asset_server);
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

fn spawn_court_house(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    y_max: LevelYMax,
) {
    let x = 0.0;
    let y = 421.0;
    let z = calculate_z(y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 65.0))
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/courthouse.png"),
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_houses(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let x = -470.0;
    let y = -75.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(33.0, 450.0))
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/houses.png"),
            transform: Transform {
                translation: Vec3::new(x, y, WALL_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_right_forest(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let x = 480.0;
    let y = 0.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, 500.0))
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/vertical_forest_0.png"),
            transform: Transform {
                translation: Vec3::new(x, y, WALL_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}


fn unload() {}