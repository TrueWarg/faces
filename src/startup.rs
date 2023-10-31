use bevy::{
    prelude::{AssetServer, Camera2dBundle, Commands, Res, Transform, Vec3},
    sprite::SpriteBundle,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::core::components::MainCamera;

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
