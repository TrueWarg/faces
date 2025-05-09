use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::UVec2;
use bevy::prelude::in_state;
use bevy::prelude::AppExtStates;
use bevy::prelude::Component;
use bevy::prelude::DetectChanges;
use bevy::prelude::NextState;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::State;
use bevy::prelude::States;
use bevy::prelude::TransformBundle;
use bevy::{
    asset::Assets,
    ecs::entity::Entity,
    ecs::query::With,
    ecs::schedule::IntoSystemConfigs,
    ecs::system::Query,
    ecs::system::ResMut,
    input::keyboard::KeyCode,
    input::ButtonInput,
    prelude::AssetServer,
    prelude::BuildChildren,
    prelude::Commands,
    prelude::Plugin,
    prelude::Res,
    prelude::Transform,
    prelude::Update,
    prelude::Vec3,
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    time::Timer,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use super::objects::wooden_chest_states_draws;
use super::objects::{interact_with_container_handle, spawn_container, LevelArm};
use super::sprites::WoodenChestSprites;
use super::COURIER_DIALOG;
use super::END_DIALOG_NECK_TWISTED;
use super::SLEEPING_FORMIDABLE_DOG_DIALOG;
use super::{
    dialog_starts, HasDialogId, END_DIALOG_AGENDA_TAKEN, END_DIALOG_FORMIDABLE_DOG_JOINED,
};
use crate::animation::entities::MoveDirection;
use crate::core::entities::BodyYOffset;
use crate::core::z_index::DEFAULT_OBJECT_Z;
use crate::dialog::SelectedVariantsSource;
use crate::interaction::interactors::{
    change_switcher_state, detect_active_interaction, ActiveInteractor,
};
use crate::level::house::FormidableDogState::Wakefulness;
use crate::level::states::Level;
use crate::npc::{spawn_fixed_npc, spawn_formidable_dog, IdleAnimation};
use crate::party::{PartyMember, PartyStateStorage};
use crate::player::entities::{FormidableDog, PlayerPosition};
use crate::rpg::{Character, CharacterStorage, ConsumableItem};
use crate::world_state::EscapeFromHouse;
use crate::world_state::EscapeFromHouse::{CallDog, Escape, GoSleep};
use crate::{
    core::{
        collisions::recalculate_z,
        entities::{Description, LevelYMax},
        state_machines::CycleLinearTransition,
        z_index::{calculate_z, FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z},
    },
    interaction::interactors::{
        InteractionArea, InteractionSide, PassiveInteractor, Switcher, SwitcherState,
    },
};

#[derive(Component)]
struct HouseLevel;

#[derive(Component)]
struct Courier;

#[derive(Component)]
struct Door;

impl HasDialogId for Courier {
    fn dialog_id(&self) -> usize {
        COURIER_DIALOG
    }
}

#[derive(Component)]
struct SleepingFormidableDog;

impl HasDialogId for SleepingFormidableDog {
    fn dialog_id(&self) -> usize {
        SLEEPING_FORMIDABLE_DOG_DIALOG
    }
}

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
        app.init_state::<FormidableDogState>()
            .add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(Update, courier_spawns.run_if(in_state(self.state.clone())))
            .add_systems(OnExit(self.state.clone()), despawn_courier)
            .add_systems(Update, interact_with_container_handle)
            .add_systems(
                Update,
                sleeping_formidable_dog_spawns.run_if(in_state(self.state.clone())),
            )
            .add_systems(OnExit(self.state.clone()), despawn_sleeping_dog)
            .add_systems(OnEnter(Wakefulness), initial_spawn_formidable_dog)
            .add_systems(
                Update,
                dialog_starts::<Courier>.run_if(in_state(EscapeFromHouse::Courier)),
            )
            .add_systems(
                Update,
                dialog_starts::<SleepingFormidableDog>.run_if(in_state(CallDog)),
            )
            .add_systems(Update, escape_from_house_handle.run_if(in_state(Escape)))
            .add_systems(
                Update,
                (
                    recalculate_z,
                    escape_from_house_variants_handles,
                    wooden_chest_states_draws::<ConsumableItem>
                        .after(interact_with_container_handle),
                    draw_level_arm_states.after(change_switcher_state),
                )
                    .run_if(in_state(self.state.clone())),
            );
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let y_max = LevelYMax::create(192.0);
    commands.spawn((y_max, HouseLevel));

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
    spawn_wooden_chest(&mut commands, &asset_server, y_max);
}

fn escape_from_house_handle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut level_state: ResMut<NextState<Level>>,
    mut next_player_position_query: Query<&mut PlayerPosition>,
    interactors_query: Query<(&PassiveInteractor, &Transform), With<Door>>,
    active_interactor_query: Query<(&ActiveInteractor, &Transform)>,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }

    for (interactor, transform) in interactors_query.iter() {
        let is_interacting =
            detect_active_interaction(&active_interactor_query, (interactor, transform));
        if is_interacting {
            let mut position = next_player_position_query.single_mut();
            position.x = 20.0;
            position.y = -400.0;
            level_state.set(Level::CourtHouseFront);
        }
    }
}

fn escape_from_house_variants_handles(
    mut character_storage: ResMut<CharacterStorage>,
    mut party_state: ResMut<PartyStateStorage>,
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
                    character_storage.add(Character::initial_formidable_dog());
                    party_state.add_party_member(PartyMember::initial_formidable_dog());
                    formidable_dog_state.set(Wakefulness);
                }
            }
            escape_from_house_state.set(Escape);
        }
    }
}

fn unload(mut commands: Commands, query: Query<Entity, With<HouseLevel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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
        },
        HouseLevel,
    );

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

fn spawn_wooden_chest(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
    let chest_y = 112.0;
    let chest_z = calculate_z(chest_y, y_max.value);
    spawn_container(
        commands,
        HouseLevel,
        asset_server.load("chest/wooden.png"),
        -145.0,
        105.0,
        chest_z,
        -2.0,
        vec![
            ConsumableItem::default_dumplings(),
            ConsumableItem::default_venison(),
        ],
    );
}

fn spawn_dog_house(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
    let doghouse_y = -136.0;
    let doghouse_z = calculate_z(doghouse_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/doghouse.png"),
                transform: Transform {
                    translation: Vec3::new(145.0, -100.0, doghouse_z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ))
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
            spawn_sleeping_formidable_dog(asset_server, commands, layouts, level_y_max);
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
    let layout = TextureAtlasLayout::from_grid(UVec2::new(27, 18), 16, 1, None, None);

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
            HouseLevel,
        ))
        .insert(IdleAnimation {
            timer: Timer::from_seconds(0.4, bevy::time::TimerMode::Repeating),
            frames_count: 16,
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

fn despawn_sleeping_dog(mut commands: Commands, query: Query<Entity, With<SleepingFormidableDog>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_bed(commands: &mut Commands, asset_server: &Res<AssetServer>, y_max: LevelYMax) {
    let bed_y = -168.0;
    let bed_z = calculate_z(bed_y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/bed.png"),
                transform: Transform {
                    translation: Vec3::new(-145.0, -120.0, bed_z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ))
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
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/vase_on_table.png"),
                transform: Transform {
                    translation: Vec3::new(-55.0, 175.0, vase_z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ))
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
        .insert((SpriteBundle {
            texture: asset_server.load("house/chest.png"),
            transform: Transform {
                translation: Vec3::new(-145.0, 155.0, chest_z),
                ..Default::default()
            },
            ..Default::default()
        }, HouseLevel))
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
    commands
        .spawn(RigidBody::Fixed)
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/door.png"),
                transform: Transform {
                    translation: Vec3::new(120.0, 224.0, ON_WALL_OBJECT_Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            Door,
            HouseLevel,
        ))
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(30.0, 32.0),
            side: InteractionSide::Bottom,
        });
}

fn spawn_floor(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(RigidBody::Fixed).insert((
        SpriteBundle {
            texture: asset_server.load("house/floor.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, FLOOR_Z),
                ..Default::default()
            },
            ..Default::default()
        },
        HouseLevel,
    ));
}

fn spawn_walls(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(32.0, 256.0))
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/wall_left.png"),
                transform: Transform {
                    translation: Vec3::new(-224.0, 0.0, WALL_Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(256.0, 32.0))
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/wall_top.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 224.0, WALL_Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(32.0, 256.0))
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/wall_right.png"),
                transform: Transform {
                    translation: Vec3::new(224.0, 0.0, WALL_Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(256.0, 32.0))
        .insert((
            SpriteBundle {
                texture: asset_server.load("house/wall_bottom.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, -224.0, WALL_Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            HouseLevel,
        ));
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
            spawn_fixed_npc(
                &asset_server,
                &mut commands,
                &mut layouts,
                (Courier, HouseLevel),
                "npc/clerk.png".to_string(),
                MoveDirection::ForwardIdle,
                120.0,
                200.0,
                ON_WALL_OBJECT_Z + 1.5,
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

fn despawn_courier(mut commands: Commands, courier_query: Query<Entity, With<Courier>>) {
    for entity in courier_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
