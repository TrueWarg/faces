use bevy::{
    prelude::{
        AssetServer, BuildChildren, Commands, Plugin, Res, Startup, Transform, Update, Vec3,
    },
    sprite::SpriteBundle,
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    core::{
        collisions::recalculate_z,
        components::{Description, LevelYMax},
        z_index::{calculate_z, FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z},
    },
    interaction::component::{InteractionArea, InteractionSide, PassiveInteractor},
};

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // currently it loaded on Startup for testing
        app.add_systems(Startup, load)
            .add_systems(Update, recalculate_z);
    }
}

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let y_max = LevelYMax::create(192.0);
    commands.spawn(y_max);

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/floor.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, FLOOR_Z),
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
                translation: Vec3::new(-224.0, 0.0, WALL_Z),
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
                translation: Vec3::new(0.0, 224.0, WALL_Z),
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
                translation: Vec3::new(224.0, 0.0, WALL_Z),
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
                translation: Vec3::new(0.0, -224.0, WALL_Z),
                ..Default::default()
            },
            ..Default::default()
        });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/door.png"),
        transform: Transform {
            translation: Vec3::new(120.0, 224.0, ON_WALL_OBJECT_Z),
            ..Default::default()
        },
        ..Default::default()
    });

    let chest_y = 131.0;
    let chest_z = calculate_z(chest_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("house/chest.png"),
            transform: Transform {
                translation: Vec3::new(-145.0, 155.0, chest_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(31.0, 18.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -6.0, chest_z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(31.0, 18.0),
            side: InteractionSide::Bottom,
        })
        .insert(Description {
            text: "Твой сундук. Сделан из титана, с замком 41-го типа. Правда, внутри пусто, т.к. кто-то всё же стащил оттуда деньги".to_string()
        });

    let vase_y = 132.0;
    let vase_z = calculate_z(vase_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("house/vase_on_table.png"),
            transform: Transform {
                translation: Vec3::new(-55.0, 175.0, vase_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(31.0, 12.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -35.0, vase_z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::create(31.0, 8.0, 0.0, -25.0),
            side: InteractionSide::Bottom,
        })
        .insert(Description {
            text: "Ваза с трещинами, много раз склеенная".to_string(),
        });

    let bed_y = -168.0;
    let bed_z = calculate_z(bed_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("house/bed.png"),
            transform: Transform {
                translation: Vec3::new(-145.0, -120.0, bed_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(33.0, 40.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -8.0, bed_z)));
        });

    let doghouse_y = -136.0;
    let doghouse_z = calculate_z(doghouse_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("house/doghouse.png"),
            transform: Transform {
                translation: Vec3::new(145.0, -100.0, doghouse_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(36.0, 20.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -16.0, doghouse_z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(36.0, 20.0),
            side: InteractionSide::Bottom,
        })
        .insert(Description {
            text: "Будка Грозного Пса. Нужно каждый раз проверять нет ли миски молока".to_string(),
        });
}
