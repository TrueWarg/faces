use bevy::app::{Plugin, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec3;
use bevy::prelude::in_state;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::SpriteBundle;
use bevy::prelude::States;
use bevy::prelude::TextureAtlasLayout;
use bevy::prelude::Transform;
use bevy::prelude::{Commands, Component, NextState};
use bevy_rapier2d::dynamics::RigidBody;

use crate::animation::entities::MoveDirection;
use crate::core::collisions::recalculate_z;
use crate::core::entities::LevelYMax;
use crate::core::z_index::{calculate_z, FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z};
use crate::dialog::SelectedVariantsSource;
use crate::level::dialog_starts;
use crate::level::objects::spawn_object;
use crate::level::HasDialogId;
use crate::level::CRAZY_MAN_DIALOG;
use crate::level::CRAZY_MAN_DIALOG_BEATEN;
use crate::level::CRAZY_MAN_DIALOG_COMPLETED;
use crate::level::HALL_GUARDIAN_FIRST_DIALOG;
use crate::level::HALL_GUARDIAN_FIRST_DIALOG_BEATEN;
use crate::level::HALL_GUARDIAN_SECOND_COMPLETED;
use crate::level::HALL_GUARDIAN_SECOND_DIALOG;
use crate::level::TABLE_1_DIALOG;
use crate::level::TABLE_2_DIALOG;
use crate::level::TABLE_3_DIALOG;
use crate::npc::spawn_fixed_npc;
use crate::world_state::GoIntoCourt;

#[derive(Component)]
struct GuardianFirstStage;

impl HasDialogId for GuardianFirstStage {
    fn dialog_id(&self) -> usize {
        HALL_GUARDIAN_FIRST_DIALOG
    }
}

#[derive(Component)]
struct GuardianSecondStage;

impl HasDialogId for GuardianSecondStage {
    fn dialog_id(&self) -> usize {
        HALL_GUARDIAN_SECOND_DIALOG
    }
}

#[derive(Component)]
struct Clerc1;

impl HasDialogId for Clerc1 {
    fn dialog_id(&self) -> usize {
        TABLE_1_DIALOG
    }
}

#[derive(Component)]
struct Clerc2;

impl HasDialogId for Clerc2 {
    fn dialog_id(&self) -> usize {
        TABLE_2_DIALOG
    }
}

#[derive(Component)]
struct Clerc3;

impl HasDialogId for Clerc3 {
    fn dialog_id(&self) -> usize {
        TABLE_3_DIALOG
    }
}

#[derive(Component)]
struct CrazyMan;

impl HasDialogId for CrazyMan {
    fn dialog_id(&self) -> usize {
        CRAZY_MAN_DIALOG
    }
}

pub struct CourtHouseHallPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for CourtHouseHallPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnEnter(self.state.clone()), spawn_guardians)
            .add_systems(OnEnter(self.state.clone()), spawn_working_clerks)
            .add_systems(OnEnter(self.state.clone()), spawn_speaking_clerks)
            .add_systems(OnEnter(self.state.clone()), spawn_visitors)
            .add_systems(OnEnter(self.state.clone()), spawn_crazy_man)
            .add_systems(
                Update,
                dialog_starts::<Clerc1>.run_if(in_state(GoIntoCourt::Wait)),
            )
            .add_systems(
                Update,
                dialog_starts::<Clerc2>.run_if(in_state(GoIntoCourt::Wait)),
            )
            .add_systems(
                Update,
                dialog_starts::<Clerc3>.run_if(in_state(GoIntoCourt::Wait)),
            )
            .add_systems(
                Update,
                dialog_starts::<GuardianFirstStage>.run_if(in_state(GoIntoCourt::Wait)),
            )
            .add_systems(
                Update,
                dialog_starts::<GuardianSecondStage>.run_if(in_state(GoIntoCourt::CanGo)),
            )
            .add_systems(
                Update,
                dialog_starts::<CrazyMan>.run_if(in_state(GoIntoCourt::Wait)),
            )
            .add_systems(
                Update,
                (dialog_variants_handles, recalculate_z).run_if(in_state(self.state.clone())),
            );
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut go_into_court_state: ResMut<NextState<GoIntoCourt>>,
) {
    go_into_court_state.set(GoIntoCourt::Wait);
    let y_max = LevelYMax::create(359.0);
    commands.spawn(y_max);

    spawn_floor(&mut commands, &asset_server);

    let wall_top = asset_server.load("courthouse_hall/wall_top.png");
    spawn_object(
        &mut commands,
        wall_top,
        0.0,
        y_max.value,
        WALL_Z,
        256.0,
        30.0,
        0.0,
    );

    spawn_doors(&mut commands, &asset_server, 0.0, 351.0);

    let wall_left = asset_server.load("courthouse_hall/wall_left.png");
    spawn_object(
        &mut commands,
        wall_left,
        -240.0,
        30.0,
        WALL_Z,
        17.0,
        359.0,
        0.0,
    );

    let wall_right = asset_server.load("courthouse_hall/wall_right.png");
    spawn_object(
        &mut commands,
        wall_right,
        240.0,
        30.0,
        WALL_Z,
        17.0,
        359.0,
        0.0,
    );

    // -------------------------------------------------------------------
    let flower = asset_server.load("courthouse_hall/flower.png");
    let x = -190.0;
    let y = 330.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 10.0, 20.0, -5.0);

    let sofa = asset_server.load("courthouse_hall/sofa_violet.png");
    let x = -190.0;
    let y = 260.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, sofa, x, y, z, 17.0, 43.0, -5.0);

    let flower = asset_server.load("courthouse_hall/flower.png");
    let x = -190.0;
    let y = 195.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 10.0, 20.0, -5.0);

    let sofa = asset_server.load("courthouse_hall/sofa_blue.png");
    let x = -190.0;
    let y = 125.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, sofa, x, y, z, 17.0, 43.0, -5.0);

    let flower = asset_server.load("courthouse_hall/flower.png");
    let x = -190.0;
    let y = 60.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 10.0, 20.0, -5.0);

    let sofa = asset_server.load("courthouse_hall/sofa_violet.png");
    let x = -190.0;
    let y = -10.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, sofa, x, y, z, 17.0, 43.0, -5.0);

    let flower = asset_server.load("courthouse_hall/flower.png");
    let x = -190.0;
    let y = -75.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 10.0, 20.0, -5.0);

    let sofa = asset_server.load("courthouse_hall/sofa_blue.png");
    let x = -190.0;
    let y = -145.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, sofa, x, y, z, 17.0, 43.0, -5.0);

    let flower = asset_server.load("courthouse_hall/flower.png");
    let x = -190.0;
    let y = -210.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 10.0, 20.0, -5.0);
    // -------------------------------------------------------------------

    // -------------------------------------------------------------------
    let flower = asset_server.load("courthouse_hall/work_place.png");
    let x = 160.0;
    let y = 190.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 15.0, 37.0, -5.0);

    let flower = asset_server.load("courthouse_hall/work_place.png");
    let x = 160.0;
    let y = 80.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 15.0, 37.0, -5.0);

    let flower = asset_server.load("courthouse_hall/work_place.png");
    let x = 160.0;
    let y = -30.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 15.0, 37.0, -5.0);
    // -------------------------------------------------------------------
}

fn spawn_floor(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("courthouse_hall/floor.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, FLOOR_Z),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_doors(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32) {
    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("courthouse_hall/doors.png"),
        transform: Transform {
            translation: Vec3::new(x, y, ON_WALL_OBJECT_Z),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_guardians(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (GuardianFirstStage, GuardianSecondStage),
        "npc/guardian.png".to_string(),
        MoveDirection::ForwardIdle,
        -50.0,
        350.0,
        ON_WALL_OBJECT_Z + 1.5,
    );

    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (GuardianFirstStage, GuardianSecondStage),
        "npc/guardian.png".to_string(),
        MoveDirection::ForwardIdle,
        50.0,
        350.0,
        ON_WALL_OBJECT_Z + 1.5,
    );
}

fn spawn_working_clerks(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(359.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);

    let x = 190.0;
    let y = 200.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Clerc3,
        "npc/clerk.png".to_string(),
        MoveDirection::LeftIdle,
        x,
        y,
        z,
    );

    let y = 90.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Clerc2,
        "npc/clerk_blond.png".to_string(),
        MoveDirection::LeftIdle,
        x,
        y,
        z,
    );

    let y = -20.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Clerc1,
        "npc/clerk.png".to_string(),
        MoveDirection::LeftIdle,
        x,
        y,
        z,
    );
}

fn spawn_speaking_clerks(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(359.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);

    let x = 160.0;
    let y = -190.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/clerk.png".to_string(),
        MoveDirection::RightIdle,
        x,
        y,
        z,
    );

    let x = 190.0;
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/clerk_blond.png".to_string(),
        MoveDirection::LeftIdle,
        x,
        y,
        z,
    );
}

fn spawn_visitors(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(359.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);

    let x = -120.0;
    let y = 190.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/woman_pink.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );

    let x = -160.0;
    let y = 150.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/clerk.png".to_string(),
        MoveDirection::RightIdle,
        x,
        y,
        z,
    );

    let x = -120.0;
    let y = 0.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/clerk.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );

    let x = -90.0;
    let y = 10.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/woman_green.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );

    let x = -150.0;
    let y = -150.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
        "npc/clerk.png".to_string(),
        MoveDirection::RightIdle,
        x,
        y,
        z,
    );
}

fn spawn_crazy_man(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(359.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);

    let x = 170.0;
    let y = 290.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        CrazyMan,
        "npc/crazy_man.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );
}

fn dialog_variants_handles(
    mut dialog_variant_source: ResMut<SelectedVariantsSource>,
    mut go_into_court_state: ResMut<NextState<GoIntoCourt>>,
) {
    // todo: it calculates on each frame.
    // make it when dialog_variant_source updates only.
    let selected = dialog_variant_source.consume(&HALL_GUARDIAN_FIRST_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == HALL_GUARDIAN_FIRST_DIALOG_BEATEN {
                    go_into_court_state.set(GoIntoCourt::Go);
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&HALL_GUARDIAN_SECOND_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == HALL_GUARDIAN_SECOND_COMPLETED {
                    go_into_court_state.set(GoIntoCourt::Go);
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&CRAZY_MAN_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == CRAZY_MAN_DIALOG_COMPLETED {
                    go_into_court_state.set(GoIntoCourt::CanGo);
                }

                if id == CRAZY_MAN_DIALOG_BEATEN {
                    go_into_court_state.set(GoIntoCourt::CanGo);
                }
            }
        }
    }
}

fn unload() {}
