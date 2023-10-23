use bevy::prelude::{
    App, AssetServer, Assets, Camera2dBundle, Commands, Component, Handle, Image, Input, KeyCode,
    Name, OnEnter, Plugin, Query, Res, ResMut, Transform, Vec2, Vec3, With, Without,
};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::time::Time;
use bevy::transform::TransformBundle;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity, KinematicCharacterController};

use crate::resources::AssetsPack;
use crate::states::GameState;

pub struct PlayGroundPlugin;

impl Plugin for PlayGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Exporation), build_map)
            .add_systems(OnEnter(GameState::Exporation), spawn_map);
    }
}

fn build_map(mut commands: Commands) {}

fn spawn_map(mut commands: Commands) {}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("wall_left.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::cuboid(128.0, 1.0));

    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("wall_left.png"),
            transform: Transform {
                translation: Vec3::new(128.0, 0.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::cuboid(128.0, 1.0));

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("bottom_floor.png"),
        transform: Transform {
            translation: Vec3::new(0.0, -88.0, 15.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("bottom_floor.png"),
        transform: Transform {
            translation: Vec3::new(128.0, -88.0, 15.0),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
}

pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,

) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up);
        let down = keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down);
        let left = keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left);
        let right = keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 {
            rb_vels.linvel.x = player.speed * (x_axis as f32);
        } else {
            rb_vels.linvel.x = 0.0;
        }

        if y_axis != 0 {
            rb_vels.linvel.y = player.speed * (y_axis as f32);
        } else {
            rb_vels.linvel.y = 0.0;
        }
    }
}

pub fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteBundle {
            texture: asset_server.load("npc.png"),
            ..Default::default()
        })
        .insert(Velocity::zero())
        .insert(TransformBundle::from(Transform::from_xyz(100.0, -100.0, 25.0)))
        .insert(Collider::cuboid(20.0, 20.0))
        // .insert(KinematicCharacterController::default())
        .insert(Player { speed: 80.0 });
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component, Debug)]
pub struct Player {
    speed: f32,
}
