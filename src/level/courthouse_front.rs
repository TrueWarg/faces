use bevy::app::{Plugin, Update};
use bevy::asset::{Assets, AssetServer};
use bevy::hierarchy::BuildChildren;
use bevy::input::ButtonInput;
use bevy::math::{UVec2, Vec3};
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::in_state;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::KeyCode;
use bevy::prelude::NextState;
use bevy::prelude::Query;
use bevy::prelude::TextureAtlas;
use bevy::prelude::Timer;
use bevy::prelude::TransformBundle;
use bevy::prelude::With;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::SpriteBundle;
use bevy::prelude::States;
use bevy::prelude::TextureAtlasLayout;
use bevy::prelude::Transform;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

use crate::core::collisions::recalculate_z;
use crate::core::entities::{BodyYOffset, LevelYMax};
use crate::core::states::GameState;
use crate::core::z_index::calculate_z;
use crate::core::z_index::DEFAULT_OBJECT_Z;
use crate::core::z_index::FLOOR_Z;
use crate::core::z_index::MIN_RANGE_Z;
use crate::core::z_index::ON_WALL_OBJECT_Z;
use crate::core::z_index::WALL_Z;
use crate::dialog::{DialogId, SelectedVariantsSource};
use crate::interaction::interactors::ActiveInteractor;
use crate::interaction::interactors::detect_active_interaction;
use crate::interaction::interactors::InteractionArea;
use crate::interaction::interactors::InteractionSide;
use crate::interaction::interactors::PassiveInteractor;
use crate::level::{BLOND_DIALOG_FIRST, DREVNIRA_DIALOG, END_DIALOG_BLOND_FIRST_ACCEPTED, END_DIALOG_DREVNIRA_BEATEN, END_DIALOG_GOPNIKS_DIALOG_ASK_BLOND, GOPNIKS_DIALOG};
use crate::npc::{IdleAnimation, spawn_npc};
use crate::world_state::{BlondAndGopniks, StrangeOldWoman};

pub struct CourtHouseFrontPlugin<S: States> {
    pub state: S,
}

#[derive(Component)]
struct Drevnira;

#[derive(Component)]
struct Blond;

#[derive(Component)]
struct Gopnik;

impl<S: States> Plugin for CourtHouseFrontPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnEnter(self.state.clone()), spawn_old_woman_drevnira)
            .add_systems(OnEnter(self.state.clone()), spawn_guardians)
            .add_systems(OnEnter(self.state.clone()), spawn_gopniks)
            .add_systems(OnEnter(self.state.clone()), spawn_blond_man)
            .add_systems(OnExit(GameState::Exploration), unload)
            .add_systems(Update, drevnira_dog_dialog_starts.run_if(in_state(StrangeOldWoman::GiveMeFeather)))
            .add_systems(Update, blond_first_dialog_starts.run_if(in_state(BlondAndGopniks::TalkWithBlond)))
            .add_systems(Update, gopniks_dialog_starts.run_if(in_state(BlondAndGopniks::TalkWithGopniks)))
            .add_systems(Update, (dialog_variants_handles, recalculate_z).run_if(in_state(self.state.clone())));
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut drevnira_state: ResMut<NextState<StrangeOldWoman>>,
    mut blond_state: ResMut<NextState<BlondAndGopniks>>,
) {
    drevnira_state.set(StrangeOldWoman::GiveMeFeather);
    blond_state.set(BlondAndGopniks::TalkWithBlond);

    let y_max = LevelYMax::create(500.0);
    commands.spawn(y_max);

    spawn_ground(&mut commands, &asset_server);

    spawn_court_house(&mut commands, &asset_server);
    spawn_court_doors(&mut commands, &asset_server);

    spawn_houses(&mut commands, &asset_server);

    spawn_right_forest(&mut commands, &asset_server);

    spawn_tree_1(&mut commands, &asset_server, y_max, 230.0, 200.0);
    spawn_tree_3(&mut commands, &asset_server, y_max, 265.0, 215.0);
    spawn_tree_1(&mut commands, &asset_server, y_max, 210.0, 220.0);
    spawn_tree_1(&mut commands, &asset_server, y_max, 253.0, 225.0);

    spawn_tree_3(&mut commands, &asset_server, y_max, 105.0, 155.0);

    spawn_tree_2(&mut commands, &asset_server, y_max, -285.0, 45.0);

    spawn_tree_2(&mut commands, &asset_server, y_max, -295.0, 295.0);
    spawn_tree_3(&mut commands, &asset_server, y_max, -280.0, 275.0);

    spawn_tree_1(&mut commands, &asset_server, y_max, -320.0, -70.0);

    spawn_tree_1(&mut commands, &asset_server, y_max, -350.0, -170.0);
    spawn_tree_3(&mut commands, &asset_server, y_max, -340.0, -150.0);

    spawn_tree_3(&mut commands, &asset_server, y_max, 235.0, -60.0);

    spawn_tree_2(&mut commands, &asset_server, y_max, 220.0, -245.0);
    spawn_tree_2(&mut commands, &asset_server, y_max, 210.0, -240.0);
    spawn_tree_3(&mut commands, &asset_server, y_max, 225.0, -255.0);
    spawn_tree_2(&mut commands, &asset_server, y_max, 210.0, -215.0);

    spawn_bench(&mut commands, &asset_server, y_max, 80.0, 0.0);

    spawn_bench(&mut commands, &asset_server, y_max, 430.0, 225.0);
    spawn_bench(&mut commands, &asset_server, y_max, 430.0, 125.0);
    spawn_bench(&mut commands, &asset_server, y_max, 430.0, 25.0);
    spawn_bench(&mut commands, &asset_server, y_max, 430.0, -75.0);
    spawn_bench(&mut commands, &asset_server, y_max, 430.0, -175.0);
    spawn_bench(&mut commands, &asset_server, y_max, 430.0, -275.0);
}

fn spawn_ground(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/ground.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, FLOOR_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_court_house(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let x = 0.0;
    let y = 421.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 65.0))
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/courthouse.png"),
            transform: Transform {
                translation: Vec3::new(x, y, WALL_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_court_doors(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let x = 0.0;
    let y = 371.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/doors.png"),
            transform: Transform {
                translation: Vec3::new(x, y, ON_WALL_OBJECT_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_houses(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let x = -470.0;
    let y = -75.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(33.0, 450.0))
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/houses.png"),
            transform: Transform {
                translation: Vec3::new(x, y, WALL_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_right_forest(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let x = 480.0;
    let y = 0.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, 500.0))
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/vertical_forest_0.png"),
            transform: Transform {
                translation: Vec3::new(x, y, MIN_RANGE_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_tree_1(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    y_max: LevelYMax,
    x: f32,
    y: f32,
) {
    let z = calculate_z(y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/tree_1.png"),
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(31.0, 20.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -5.0, z,
                )));
        });
}

fn spawn_tree_2(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    y_max: LevelYMax,
    x: f32,
    y: f32,
) {
    let z = calculate_z(y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/tree_2.png"),
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(23.0, 20.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -5.0, z,
                )));
        });
}

fn spawn_tree_3(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    y_max: LevelYMax,
    x: f32,
    y: f32,
) {
    let z = calculate_z(y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/tree_3.png"),
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(24.0, 20.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -5.0, z,
                )));
        });
}

fn spawn_bench(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    y_max: LevelYMax,
    x: f32,
    y: f32,
) {
    let z = calculate_z(y, y_max.value);
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_front/bench.png"),
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(20.0, 40.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, -2.0, z,
                )));
        });
}

fn spawn_old_woman_drevnira(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(500.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);
    let x = -430.0;
    let y = -250.0;
    let z = calculate_z(y, y_max.value);

    let image_handle = asset_server.load("npc/old_woman_drevnira.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(29, 64), 8, 1, None, None,
    );

    let layout_handle = layouts.add(layout);

    commands
        .spawn(RigidBody::Fixed)
        .insert(Drevnira)
        .insert((
            SpriteBundle {
                texture: image_handle,
                ..Default::default()
            },
            TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
        ))
        .insert(IdleAnimation {
            timer: Timer::from_seconds(
                0.7,
                bevy::time::TimerMode::Repeating,
            ),
        })
        .insert(TransformBundle::from(Transform::from_xyz(x, y, z)))
        .insert(BodyYOffset::create(30.0))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(16.0, 32.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    -8.0,
                    DEFAULT_OBJECT_Z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(16.0, 32.0),
            side: InteractionSide::Bottom,
        });
}


fn spawn_guardians(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/guardian.png".to_string(),
        -50.0,
        370.0,
        ON_WALL_OBJECT_Z + 1.5,
        0.0,
        0.0,
    );

    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/guardian.png".to_string(),
        50.0,
        370.0,
        ON_WALL_OBJECT_Z + 1.5,
        0.0,
        0.0,
    );
}

fn spawn_gopniks(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(500.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);

    let x = 360.0;
    let y = 250.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        x,
        y,
        z,
        0.0,
        0.0,
    );

    let x = 380.0;
    let y = 255.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        x,
        y,
        z,
        0.0,
        0.0,
    );

    let x = 310.0;
    let y = 250.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        x,
        y,
        z,
        0.0,
        0.0,
    );

    let x = 335.0;
    let y = 220.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        x,
        y,
        z,
        0.0,
        0.0,
    );
}

fn spawn_blond_man(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(500.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);

    let x = 100.0;
    let y = -80.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Blond,
        "npc/clerk_blond.png".to_string(),
        x,
        y,
        z,
        0.0,
        0.0,
    );
}

fn drevnira_dog_dialog_starts(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform), With<Drevnira>>,
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
                Ok(mut dialog_id) => dialog_id.0 = DREVNIRA_DIALOG,
                Err(_) => {
                    commands.spawn(DialogId(DREVNIRA_DIALOG));
                }
            }
            next_game_state.set(GameState::Dialog);
        }
    }
}

fn blond_first_dialog_starts(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform), With<Blond>>,
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
                Ok(mut dialog_id) => dialog_id.0 = BLOND_DIALOG_FIRST,
                Err(_) => {
                    commands.spawn(DialogId(BLOND_DIALOG_FIRST));
                }
            }
            next_game_state.set(GameState::Dialog);
        }
    }
}

fn gopniks_dialog_starts(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform), With<Gopnik>>,
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
                Ok(mut dialog_id) => dialog_id.0 = GOPNIKS_DIALOG,
                Err(_) => {
                    commands.spawn(DialogId(GOPNIKS_DIALOG));
                }
            }
            next_game_state.set(GameState::Dialog);
        }
    }
}

fn dialog_variants_handles(
    mut dialog_variant_source: ResMut<SelectedVariantsSource>,
    mut drevnira_state: ResMut<NextState<StrangeOldWoman>>,
    mut blond_state: ResMut<NextState<BlondAndGopniks>>,
) {
    // todo: it calculates on each frame.
    // make it when dialog_variant_source updates only.
    let selected = dialog_variant_source.consume(&DREVNIRA_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_DREVNIRA_BEATEN {
                    drevnira_state.set(StrangeOldWoman::Beaten);
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&BLOND_DIALOG_FIRST);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_BLOND_FIRST_ACCEPTED {
                    blond_state.set(BlondAndGopniks::TalkWithGopniks);
                } else {
                    blond_state.set(BlondAndGopniks::Completed);
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&GOPNIKS_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_GOPNIKS_DIALOG_ASK_BLOND {
                    blond_state.set(BlondAndGopniks::TakeDumplingsFromBlond);
                } else {
                    blond_state.set(BlondAndGopniks::GiveDumplingsToBlond);
                }
            }
        }
    }
}

fn unload() {}