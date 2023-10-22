use bevy::prelude::{
    App, AssetServer, Assets, Camera2dBundle, Commands, Component, Handle, Image, Input, KeyCode,
    Name, OnEnter, Plugin, Query, Res, ResMut, Transform, Vec2, Vec3, With, Without,
};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::time::Time;

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
    let wall_left = commands
        .spawn(SpriteBundle {
            texture: asset_server.load("wall_left.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    commands.entity(wall_left).insert(TileCollider);

    let mut shift_x = 64.0 + 32.0;
    for i in 0..2 {
        let wall = commands
            .spawn(SpriteBundle {
                texture: asset_server.load("wall.png"),
                transform: Transform {
                    translation: Vec3::new(shift_x, 0.0, 20.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        commands.entity(wall).insert(TileCollider);
        shift_x += 64.0;
    }

    let mut shift_x = -(64.0 - 16.0);
    for i in 0..8 {
        let idx = 2 & i;
        let asset_path = format!("floor_{}.png", idx);
        commands.spawn(SpriteBundle {
            texture: asset_server.load(asset_path),
            transform: Transform {
                translation: Vec3::new(shift_x, -(32.0 + 12.0), 15.0),
                ..Default::default()
            },
            ..Default::default()
        });
        shift_x += 32.0;
    }

    for i in 0..2 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("bottom_floor.png"),
            transform: Transform {
                translation: Vec3::new((128 * i) as f32, -(59.0 + 58.0 / 2.0 + 16.0 + 12.0), 15.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
}

pub fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

pub fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    let player = commands
        .spawn(SpriteBundle {
            texture: asset_server.load("npc.png"),
            transform: Transform {
                translation: Vec3::new(100.0, -100.0, 25.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 });
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct TileCollider;

const TILE_SIZE: f32 = 50.0;

#[derive(Component, Debug)]
pub struct Player {
    speed: f32,
}
