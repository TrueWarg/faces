use bevy::{
    prelude::{AssetServer, Commands, Plugin, Res, Startup, Transform, Vec3},
    sprite::SpriteBundle,
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // currently it loaded on Startup for testing
        app.add_systems(Startup, load);
    }
}

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/floor.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 20.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(32.0, 256.0))
        .insert(SpriteBundle {
            texture: asset_server.load("house/wall_left.png"),
            transform: Transform {
                translation: Vec3::new(-224.0, 0.0, 25.0),
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(256.0, 32.0))
        .insert(SpriteBundle {
            texture: asset_server.load("house/wall_top.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 224.0, 25.0),
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(32.0, 256.0))
        .insert(SpriteBundle {
            texture: asset_server.load("house/wall_right.png"),
            transform: Transform {
                translation: Vec3::new(224.0, 0.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(256.0, 32.0))
        .insert(SpriteBundle {
            texture: asset_server.load("house/wall_bottom.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -224.0, 25.0),
                ..Default::default()
            },
            ..Default::default()
        });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/door.png"),
        transform: Transform {
            translation: Vec3::new(120.0, 224.0, 26.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/chest.png"),
        transform: Transform {
            translation: Vec3::new(-145.0, 155.0, 26.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/vase_on_table.png"),
        transform: Transform {
            translation: Vec3::new(-55.0, 175.0, 26.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/bed.png"),
        transform: Transform {
            translation: Vec3::new(-145.0, -120.0, 26.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/doghouse.png"),
        transform: Transform {
            translation: Vec3::new(145.0, -100.0, 26.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
