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
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::UVec2;
use bevy::prelude::{AppExtStates, Component, DetectChanges, in_state, NextState, OnEnter, OnExit, State, States, Time, TransformBundle};
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
use crate::core::entities::BodyYOffset;
use crate::core::states::GameState;
use crate::core::z_index::DEFAULT_OBJECT_Z;
use crate::dialog::{DialogId, SelectedVariantsSource};
use crate::interaction::interactors::ActiveInteractor;
use crate::interaction::interactors::change_switcher_state;
use crate::interaction::interactors::detect_active_interaction;
use crate::interaction::interactors::transit_to_next_container_state;
use crate::level::house::FormidableDogState::Wakefulness;
use crate::movement::routes::route_build;
use crate::npc::{IdleAnimation, move_agent_moves, npc_animation, npc_basic_animation, spawn_formidable_dog, spawn_npc};
use crate::world_state::EscapeFromHouse;
use crate::world_state::EscapeFromHouse::{CallDog, Escape, GoSleep};

use super::{END_DIALOG_AGENDA_TAKEN, END_DIALOG_FORMIDABLE_DOG_JOINED};
use super::COURIER_DIALOG;
use super::END_DIALOG_NECK_TWISTED;
use super::objects::LevelArm;
use super::objects::WoodenChest;
use super::SLEEPING_FORMIDABLE_DOG_DIALOG;
use super::sprites::WoodenChestSprites;

#[derive(Component)]
struct HouseLevel;

#[derive(Component)]
struct Courier;

#[derive(Component)]
struct SleepingFormidableDog;

#[derive(Component)]
struct FormidableDog;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum FormidableDogState {
    #[default]
    Sleep,
    Wakefulness,
}

pub struct HousePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for HousePlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_state::<FormidableDogState>()
            .add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnExit(GameState::Exploration), unload)
            .add_systems(Update, courier_spawns.run_if(in_state(self.state.clone())))
            .add_systems(OnExit(self.state.clone()), despawn_courier)
            .add_systems(Update, sleeping_formidable_dog_spawns.run_if(in_state(self.state.clone())))
            .add_systems(OnExit(self.state.clone()), despawn_sleeping_dog)
            .add_systems(OnEnter(Wakefulness), initial_spawn_formidable_dog)
            .add_systems(Update, route_build)
            .add_systems(Update, move_agent_moves.after(route_build))
            .add_systems(Update, npc_animation.after(move_agent_moves))
            .add_systems(Update, npc_basic_animation)
            .add_systems(Update, courier_dialog_starts.run_if(in_state(EscapeFromHouse::Courier)))
            .add_systems(Update, formidable_dog_dialog_starts.run_if(in_state(CallDog)))
            .add_systems(Update, sleeping_dog_basic_animation.run_if(in_state(FormidableDogState::Sleep)))
            .add_systems(
                Update,
                (recalculate_z,
                 escape_from_house_variants_handles,
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

fn escape_from_house_variants_handles(
    mut dialog_variant_source: ResMut<SelectedVariantsSource>,
    mut escape_from_house_state: ResMut<NextState<EscapeFromHouse>>,
    mut formidable_dog_state: ResMut<NextState<FormidableDogState>>,
) {
    // todo: it calculates on each frame.
    // make it when dialog_variant_source updates only.
    let selected = dialog_variant_source.consume(&COURIER_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_NECK_TWISTED {
                    escape_from_house_state.set(GoSleep)
                }

                if id == END_DIALOG_AGENDA_TAKEN {
                    escape_from_house_state.set(CallDog)
                }
            }
        }
    }
    let selected = dialog_variant_source.consume(&SLEEPING_FORMIDABLE_DOG_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_FORMIDABLE_DOG_JOINED {
                    escape_from_house_state.set(Escape);
                    formidable_dog_state.set(Wakefulness);
                }
            }
        }
    }
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

fn sleeping_formidable_dog_spawns(
    asset_server: Res<AssetServer>,
    commands: Commands,
    layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
    formidable_dog_state: Res<State<FormidableDogState>>,
    query: Query<Entity, With<SleepingFormidableDog>>,
) {
    if !formidable_dog_state.is_changed() {
        return;
    }

    match formidable_dog_state.get() {
        FormidableDogState::Sleep => {
            spawn_sleeping_formidable_dog(
                asset_server, commands, layouts, level_y_max,
            );
        }
        Wakefulness => {
            despawn_sleeping_dog(commands, query);
        }
    }
}

fn spawn_sleeping_formidable_dog(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(192.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);
    let x = 145.0;
    let y = -125.0;
    let z = calculate_z(-137.0, y_max.value);

    let image_handle = asset_server.load("npc/formidable_dog_sleeping.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(27, 18), 16, 1, None, None,
    );

    let layout_handle = layouts.add(layout);

    commands
        .spawn(RigidBody::Fixed)
        .insert((
            SpriteBundle {
                texture: image_handle,
                ..Default::default()
            },
            TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
            SleepingFormidableDog,
        ))
        .insert(IdleAnimation {
            timer: Timer::from_seconds(
                0.4,
                bevy::time::TimerMode::Repeating,
            ),
        })
        .insert(TransformBundle::from(Transform::from_xyz(x, y, z)))
        .insert(BodyYOffset::create(20.0))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(16.0, 8.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    -16.0,
                    DEFAULT_OBJECT_Z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(16.0, 8.0),
            side: InteractionSide::Bottom,
        });
}

fn despawn_sleeping_dog(
    mut commands: Commands,
    query: Query<Entity, With<SleepingFormidableDog>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn sleeping_dog_basic_animation(
    time: Res<Time>,
    mut animation_query: Query<(&mut IdleAnimation, &mut TextureAtlas)>,
) {
    for (mut idle_animation, mut sprite) in animation_query.iter_mut() {
        idle_animation.timer.tick(time.delta());
        if idle_animation.timer.finished() {
            if sprite.index >= /*15*/ 7  {
                sprite.index = 0;
            } else {
                sprite.index += 1;
            }
        }
    }
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

fn courier_dialog_starts(
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

fn formidable_dog_dialog_starts(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform), With<SleepingFormidableDog>>,
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
                Ok(mut dialog_id) => dialog_id.0 = SLEEPING_FORMIDABLE_DOG_DIALOG,
                Err(_) => {
                    commands.spawn(DialogId(SLEEPING_FORMIDABLE_DOG_DIALOG));
                }
            }
            next_game_state.set(GameState::Dialog);
        }
    }
}

fn courier_spawns(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    current_screen_state: Res<State<EscapeFromHouse>>,
    courier_query: Query<Entity, With<Courier>>,
) {
    if !current_screen_state.is_changed() {
        return;
    }
    match current_screen_state.get() {
        EscapeFromHouse::Courier => {
            spawn_npc(
                &asset_server,
                &mut commands,
                &mut layouts,
                Courier,
                "npc/clerk.png".to_string(),
                120.0,
                200.0,
                ON_WALL_OBJECT_Z + 1.5,
                0.0,
                0.0,
            );
        }
        _ => {
            despawn_courier(commands, courier_query);
        }
    }
}

fn initial_spawn_formidable_dog(
    asset_server: Res<AssetServer>,
    commands: Commands,
    layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let x = 105.0;
    let y = -145.0;
    let z = DEFAULT_OBJECT_Z;
    spawn_formidable_dog(
        asset_server,
        commands,
        layouts,
        FormidableDog,
        x,
        y,
        z,
        200.0,
        0.0,
    )
}

fn despawn_courier(
    mut commands: Commands,
    courier_query: Query<Entity, With<Courier>>,
) {
    for entity in courier_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}