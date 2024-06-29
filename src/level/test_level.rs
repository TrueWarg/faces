use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        query::With
        ,
        system::{Query, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    prelude::{
        AssetServer, Commands, Plugin, Res, Startup, Transform, Update, Vec3,
    },
    sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    time::Timer
    ,
};

use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, States};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    core::{
        collisions::recalculate_z,
        components::{Description, LevelYMax},
        state_machines::CycleLinearTransition,
        z_index::{FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z},
    },
    interaction::component::{
        Container, ContainerState, InteractionArea, InteractionSide,
        PassiveInteractor, Switcher, SwitcherState,
    },
};

use super::{
    component::{LevelArm, WoodenChest},
    resources::WoodenChestSprites,
};

pub struct TestLevel<S: States> {
    pub state: S,
}

impl<S: States> Plugin for TestLevel<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(
                Update,
                (recalculate_z).run_if(in_state(self.state.clone())),
            );
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let y_max = LevelYMax::create(192.0);
    commands.spawn(y_max);

    let wooden_chest_sprites = WoodenChestSprites {
        closed: asset_server.load("chest/wooden.png"),
        full: asset_server.load("chest/wooden_full.png"),
        empty: asset_server.load("chest/wooden_empty.png"),
    };

    commands.insert_resource(wooden_chest_sprites);

    spawn_floor(&mut commands, &asset_server);
    spawn_walls(&mut commands, &asset_server);
    spawn_door(&mut commands, &asset_server);
    spawn_level_arm(&mut commands, &asset_server, texture_atlases);
}

fn unload() {}

fn draw_level_arm_states(mut switchers: Query<(&mut TextureAtlas, &Switcher), With<LevelArm>>) {
    for (mut sprite, swticher) in switchers.iter_mut() {
        sprite.index = match swticher.state {
            SwitcherState::Off => 0,
            SwitcherState::On => 2,
            _ => 1,
        };
    }
}

fn spawn_level_arm(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("house/tileset_level_arm.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(10.0, 34.0), 3, 1, None, None);
    let layout_handle = layouts.add(layout);

    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
            transform: Transform {
                translation: Vec3::new(20.0, 215.0, ON_WALL_OBJECT_Z),
                ..Default::default()
            },
            texture: texture_handle,
            ..Default::default()
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(5.0, 17.0),
            side: InteractionSide::Bottom,
        })
        .insert(LevelArm)
        .insert(Switcher {
            timer: Timer::from_seconds(0.05, bevy::time::TimerMode::Once),
            state: SwitcherState::initial_state(),
        })
        .insert(Description {
            text: "Level arm".to_string(),
        });
}

fn draw_wooden_chest_states(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    sprites: Res<WoodenChestSprites>,
    chests: Query<(Entity, &Container), With<WoodenChest>>,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }
    for (entity, container) in chests.iter() {
        let new_sprite = match container.state {
            ContainerState::Closed => sprites.closed.clone(),
            ContainerState::Full => sprites.full.clone(),
            ContainerState::Empty => sprites.empty.clone(),
        };
        commands.entity(entity).insert(new_sprite);
    }
}

fn spawn_door(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/door.png"),
        transform: Transform {
            translation: Vec3::new(120.0, 224.0, ON_WALL_OBJECT_Z),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_floor(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("house/floor.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, FLOOR_Z),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_walls(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
}
