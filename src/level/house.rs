use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode}
    ,
    prelude::{
        AssetServer, BuildChildren, Commands, Plugin, Res, Transform, Update, Vec3,
    },
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    time::Timer,
};
use bevy::math::UVec2;
use bevy::prelude::{Component, in_state, NextState, OnEnter, OnExit, States, Time, TransformBundle};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    core::{
        collisions::recalculate_z,
        entities::{Description, LevelYMax},
        state_machines::{CycleLinearTransition, FiniteLinearTransition},
        z_index::{calculate_z, FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z},
    },
    interaction::interactors::{
        Container, ContainerState, InteractionArea, InteractionSide, LimitedInteractor,
        PassiveInteractor, Switcher, SwitcherState,
    },
};
use crate::core::states::GameState;
use crate::dialog::DialogId;
use crate::interaction::interactors::{ActiveInteractor, change_switcher_state, detect_active_interaction, transit_to_next_container_state};
use crate::npc::{npc_basic_animation, NpcAnimations, spawns_npc};
use crate::world_state::EscapeFromHouse;

use super::{COURIER_DIALOG, objects::{LevelArm, WoodenChest}, sprites::WoodenChestSprites};

#[derive(Component)]
struct HouseLevel;

#[derive(Component)]
struct Courier;

pub struct HousePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for HousePlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnExit(GameState::Exploration), unload)
            .add_systems(OnEnter(EscapeFromHouse::Courier), spawn_courier)
            .add_systems(OnExit(EscapeFromHouse::Courier), despawn_courier)
            .add_systems(Update, npc_basic_animation.run_if(in_state(EscapeFromHouse::Courier)))
            .add_systems(Update, dialog_starts.run_if(in_state(EscapeFromHouse::Courier)))
            .add_systems(
                Update,
                (recalculate_z,
                 draw_wooden_chest_states.after(transit_to_next_container_state),
                 draw_level_arm_states.after(change_switcher_state),
                ).run_if(in_state(self.state.clone())),
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
    spawn_chest(&mut commands, &asset_server, y_max);
    spawn_vase_on_table(&mut commands, &asset_server, y_max);
    spawn_bed(&mut commands, &asset_server, y_max);
    spawn_dog_house(&mut commands, &asset_server, y_max);
    spawn_level_arm(&mut commands, &asset_server, texture_atlases);
    spawn_test_chest(&mut commands, &asset_server, y_max);
}

fn unload(
    mut commands: Commands,
    query: Query<Entity>,
) {
    // for entity in query.iter() {
    //     println!("!!!! kek");
    //     commands.entity(entity).despawn();
    // }
}

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
    let layout = TextureAtlasLayout::from_grid(UVec2::new(10, 34), 3, 1, None, None);
    let layout_handle = layouts.add(layout);

    let bundle = (
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(20.0, 215.0, ON_WALL_OBJECT_Z),
                ..Default::default()
            },
            texture: texture_handle,
            ..Default::default()
        },
        TextureAtlas {
            layout: layout_handle,
            index: 0,
        });

    commands
        .spawn(RigidBody::Fixed)
        .insert(bundle)
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

fn spawn_test_chest(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
    let chest_y = 112.0;
    let chest_z = calculate_z(chest_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("chest/wooden.png"),
            transform: Transform {
                translation: Vec3::new(-145.0, 105.0, chest_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(16.0, 11.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -2.0, chest_z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(16.0, 11.0),
            side: InteractionSide::Bottom,
        })
        .insert(LimitedInteractor)
        .insert(WoodenChest)
        .insert(Container {
            state: ContainerState::initial_state(),
        })
        .insert(Description {
            text: "Closed chest".to_string(),
        });
}

fn spawn_dog_house(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
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

fn spawn_bed(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
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
}

fn spawn_vase_on_table(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
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
}

fn spawn_chest(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
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
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
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

fn dialog_starts(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform), With<Courier>>,
    mut dialog_id_query: Query<(&mut DialogId)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }
    for (interactor, transform) in interactors.iter() {
        let is_interacting = detect_active_interaction(&active, (interactor, transform));
        if is_interacting {
            match dialog_id_query.get_single_mut() {
                Ok(mut dialog_id) => dialog_id.0 = COURIER_DIALOG,
                Err(_) => {
                    commands.spawn(DialogId(COURIER_DIALOG));
                }
            }
            next_game_state.set(GameState::Dialog);
        }
    }
}

fn spawn_courier(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<NpcAnimations>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawns_npc(
        asset_server,
        commands,
        character_animations,
        layouts,
        Courier,
        "npc/clerk.png".to_string(),
        120.0,
        200.0,
        ON_WALL_OBJECT_Z + 1.5,
        0.0,
        0.0,
    );
}

fn despawn_courier() {}
