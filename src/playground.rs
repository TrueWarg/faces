use bevy::prelude::{
    App, AssetServer, Assets, Camera2dBundle, Commands, Component, Handle, Image, OnEnter, Plugin,
    Res, ResMut, Transform, Vec2, Vec3,
};
use bevy::sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};

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

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct TileCollider;
