use bevy::app::{Plugin, Update};
use bevy::asset::{Assets, AssetServer};
use bevy::hierarchy::BuildChildren;
use bevy::math::{UVec2, Vec3};
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::in_state;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::NextState;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::SpriteBundle;
use bevy::prelude::States;
use bevy::prelude::TextureAtlas;
use bevy::prelude::TextureAtlasLayout;
use bevy::prelude::Timer;
use bevy::prelude::Transform;
use bevy::prelude::TransformBundle;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;
use crate::animation::entities::MoveDirection;

use crate::core::collisions::recalculate_z;
use crate::core::entities::{BodyYOffset, LevelYMax};
use crate::core::states::GameState;
use crate::core::z_index::calculate_z;
use crate::core::z_index::DEFAULT_OBJECT_Z;
use crate::core::z_index::FLOOR_Z;
use crate::core::z_index::MIN_RANGE_Z;
use crate::core::z_index::ON_WALL_OBJECT_Z;
use crate::core::z_index::WALL_Z;
use crate::dialog::SelectedVariantsSource;
use crate::interaction::interactors::InteractionArea;
use crate::interaction::interactors::InteractionSide;
use crate::interaction::interactors::PassiveInteractor;
use crate::level::{BLOND_FIRST_DIALOG, dialog_starts, HasDialogId};
use crate::level::BLOND_GIVE_DUMPLINGS_DIALOG;
use crate::level::BLOND_TAKE_DUMPLINGS_DIALOG;
use crate::level::DREVNIRA_DIALOG;
use crate::level::END_DIALOG_BLOND_FIRST_ACCEPTED;
use crate::level::END_DIALOG_BLOND_GIVE_DUMPLINGS_COMPLETED;
use crate::level::END_DIALOG_BLOND_TAKE_DUMPLINGS_JUST_COMPLETED;
use crate::level::END_DIALOG_BLOND_TAKE_DUMPLINGS_NECK_TWISTED;
use crate::level::END_DIALOG_DREVNIRA_BEATEN;
use crate::level::END_DIALOG_GOPNIKS_DIALOG_ASK_BLOND;
use crate::level::END_DIALOG_GUARDIAN_FIRST_BEATEN;
use crate::level::END_DIALOG_GUARDIAN_FIRST_DREVNIRA_STOP_ACCEPTED;
use crate::level::END_DIALOG_GUARDIAN_FIRST_JUST_COMPLETED;
use crate::level::END_DIALOG_GUARDIAN_SECOND_BEATEN;
use crate::level::END_DIALOG_GUARDIAN_SECOND_COMPLETED;
use crate::level::END_DIALOG_GUARDIAN_THIRD_BEATEN;
use crate::level::END_DIALOG_GUARDIAN_THIRD_COMPLETED;
use crate::level::GOPNIKS_DIALOG;
use crate::level::GUARDIAN_FIRST_DIALOG;
use crate::level::GUARDIAN_SECOND_DIALOG;
use crate::level::GUARDIAN_THIRD_DIALOG;
use crate::level::objects::spawn_object;
use crate::npc::{IdleAnimation, spawn_fixed_npc};
use crate::world_state::{BlondAndGopniks, Court, StrangeOldWoman};

pub struct CourtHouseFrontPlugin<S: States> {
    pub state: S,
}

#[derive(Component)]
struct Drevnira;

impl HasDialogId for Drevnira {
    fn dialog_id(&self) -> usize {
        return DREVNIRA_DIALOG
    }
}

#[derive(Component)]
struct BlondStart;

impl HasDialogId for BlondStart {
    fn dialog_id(&self) -> usize {
        return BLOND_FIRST_DIALOG
    }
}

#[derive(Component)]
struct BlondGiveDumplings;

impl HasDialogId for BlondGiveDumplings {
    fn dialog_id(&self) -> usize {
        return BLOND_GIVE_DUMPLINGS_DIALOG
    }
}

#[derive(Component)]
struct BlondTakeDumplings;

impl HasDialogId for BlondTakeDumplings {
    fn dialog_id(&self) -> usize {
        return BLOND_TAKE_DUMPLINGS_DIALOG
    }
}

#[derive(Component)]
struct Gopnik;

impl HasDialogId for Gopnik {
    fn dialog_id(&self) -> usize {
        return GOPNIKS_DIALOG
    }
}

#[derive(Component)]
struct GuardianFirstStage;

impl HasDialogId for GuardianFirstStage {
    fn dialog_id(&self) -> usize {
        return GUARDIAN_FIRST_DIALOG
    }
}

#[derive(Component)]
struct GuardianSecondStage;

impl HasDialogId for GuardianSecondStage {
    fn dialog_id(&self) -> usize {
        return GUARDIAN_SECOND_DIALOG
    }
}

#[derive(Component)]
struct GuardianThirdStage;

impl HasDialogId for GuardianThirdStage {
    fn dialog_id(&self) -> usize {
        return GUARDIAN_THIRD_DIALOG
    }
}

impl<S: States> Plugin for CourtHouseFrontPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnEnter(self.state.clone()), spawn_old_woman_drevnira)
            .add_systems(OnEnter(self.state.clone()), spawn_guardians)
            .add_systems(OnEnter(self.state.clone()), spawn_gopniks)
            .add_systems(OnEnter(self.state.clone()), spawn_blond_man)
            .add_systems(OnExit(GameState::Exploration), unload)
            .add_systems(Update, dialog_starts::<Drevnira>.run_if(in_state(StrangeOldWoman::GiveMeFeather)))
            .add_systems(Update, dialog_starts::<BlondStart>.run_if(in_state(BlondAndGopniks::TalkWithBlond)))
            .add_systems(Update, dialog_starts::<Gopnik>.run_if(in_state(BlondAndGopniks::TalkWithGopniks)))
            .add_systems(Update, dialog_starts::<BlondGiveDumplings>.run_if(in_state(BlondAndGopniks::GiveDumplingsToBlond)))
            .add_systems(Update, dialog_starts::<BlondTakeDumplings>.run_if(in_state(BlondAndGopniks::TakeDumplingsFromBlond)))
            .add_systems(Update, dialog_starts::<GuardianFirstStage>.run_if(in_state(Court::TalkWithGuardian)))
            .add_systems(Update, dialog_starts::<GuardianSecondStage>.run_if(in_state(Court::StopDrevnira)))
            .add_systems(Update, dialog_starts::<GuardianThirdStage>.run_if(in_state(Court::DrevniraStopped)))
            .add_systems(Update, (dialog_variants_handles, recalculate_z).run_if(in_state(self.state.clone())));
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut drevnira_state: ResMut<NextState<StrangeOldWoman>>,
    mut blond_state: ResMut<NextState<BlondAndGopniks>>,
    mut court_state: ResMut<NextState<Court>>,
) {
    drevnira_state.set(StrangeOldWoman::GiveMeFeather);
    blond_state.set(BlondAndGopniks::TalkWithBlond);
    court_state.set(Court::TalkWithGuardian);

    let y_max = LevelYMax::create(500.0);
    commands.spawn(y_max);

    spawn_ground(&mut commands, &asset_server);

    let courthouse = asset_server.load("courthouse_front/courthouse.png");
    spawn_object(&mut commands, courthouse, 0.0, 421.0, WALL_Z, 500.0, 65.0, 0.0);

    spawn_court_doors(&mut commands, &asset_server, 0.0, 371.0);

    let left_houses = asset_server.load("courthouse_front/houses.png");
    spawn_object(&mut commands, left_houses, -470.0, -75.0, WALL_Z, 33.0, 450.0, 0.0);

    let right_forest = asset_server.load("courthouse_front/vertical_forest_0.png");
    spawn_object(&mut commands, right_forest, 480.0, 0.0, MIN_RANGE_Z, 20.0, 500.0, 0.0);

    let tree_1 = asset_server.load("courthouse_front/tree_1.png");
    let x = 230.0;
    let y = 200.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_1, x, y, z, 31.0, 20.0, -5.0);

    // -------------------------------------------------------------------
    let tree_3 = asset_server.load("courthouse_front/tree_3.png");
    let x = 265.0;
    let y = 215.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_3, x, y, z, 24.0, 20.0, -5.0);

    let tree_1 = asset_server.load("courthouse_front/tree_1.png");
    let x = 210.0;
    let y = 220.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_1, x, y, z, 31.0, 20.0, -5.0);

    let tree_1 = asset_server.load("courthouse_front/tree_1.png");
    let x = 253.0;
    let y = 225.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_1, x, y, z, 31.0, 20.0, -5.0);
    // -------------------------------------------------------------------

    let tree_3 = asset_server.load("courthouse_front/tree_3.png");
    let x = 95.0;
    let y = 165.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_3, x, y, z, 24.0, 20.0, -5.0);

    let tree_2 = asset_server.load("courthouse_front/tree_2.png");
    let x = 105.0;
    let y = 155.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_2, x, y, z, 23.0, 20.0, -5.0);

    // -------------------------------------------------------------------
    let tree_2 = asset_server.load("courthouse_front/tree_2.png");
    let x = -295.0;
    let y = 295.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_2, x, y, z, 23.0, 20.0, -5.0);

    let tree_3 = asset_server.load("courthouse_front/tree_3.png");
    let x = -280.0;
    let y = 275.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_3, x, y, z, 24.0, 20.0, -5.0);
    // -------------------------------------------------------------------

    let tree_1 = asset_server.load("courthouse_front/tree_1.png");
    let x = -320.0;
    let y = -70.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_1, x, y, z, 31.0, 20.0, -5.0);

    // -------------------------------------------------------------------
    let tree_1 = asset_server.load("courthouse_front/tree_1.png");
    let x = -350.0;
    let y = -170.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_1, x, y, z, 31.0, 20.0, -5.0);

    let tree_3 = asset_server.load("courthouse_front/tree_3.png");
    let x = -340.0;
    let y = -150.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_3, x, y, z, 24.0, 20.0, -5.0);
    // -------------------------------------------------------------------

    let tree_3 = asset_server.load("courthouse_front/tree_3.png");
    let x = 235.0;
    let y = -60.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_3, x, y, z, 24.0, 20.0, -5.0);

    // -------------------------------------------------------------------
    let tree_2 = asset_server.load("courthouse_front/tree_2.png");
    let x = 220.0;
    let y = -245.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_2, x, y, z, 23.0, 20.0, -5.0);

    let tree_2 = asset_server.load("courthouse_front/tree_2.png");
    let x = 210.0;
    let y = -240.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_2, x, y, z, 23.0, 20.0, -5.0);

    let tree_3 = asset_server.load("courthouse_front/tree_3.png");
    let x = 225.0;
    let y = -255.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_3, x, y, z, 24.0, 20.0, -5.0);

    let tree_2 = asset_server.load("courthouse_front/tree_2.png");
    let x = 210.0;
    let y = -215.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tree_2, x, y, z, 23.0, 20.0, -5.0);
    // -------------------------------------------------------------------

    let bench = asset_server.load("courthouse_front/bench.png");
    let x = 80.0;
    let y = 0.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);

    let x = 430.0;
    let y = 225.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);

    let y = 125.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);

    let y = 25.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);

    let y = -75.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);

    let y = -175.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);

    let y = -275.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, bench.clone(), x, y, z, 20.0, 40.0, -2.0);
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

fn spawn_court_doors(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
) {
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

fn spawn_old_woman_drevnira(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(500.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);
    let x = -430.0;
    let y = 250.0;
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
            frames_count: 8,
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
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (GuardianFirstStage, GuardianSecondStage, GuardianThirdStage),
        "npc/guardian.png".to_string(),
        MoveDirection::ForwardIdle,
        -50.0,
        370.0,
        ON_WALL_OBJECT_Z + 1.5,
    );

    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (GuardianFirstStage, GuardianSecondStage, GuardianThirdStage),
        "npc/guardian.png".to_string(),
        MoveDirection::ForwardIdle,
        50.0,
        370.0,
        ON_WALL_OBJECT_Z + 1.5,
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
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );

    let x = 380.0;
    let y = 255.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );

    let x = 310.0;
    let y = 250.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );

    let x = 335.0;
    let y = 220.0;
    let shifted_y = y - 20.0;
    let z = calculate_z(shifted_y, y_max.value);
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        Gopnik,
        "npc/gopnik_red.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
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
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (BlondStart, BlondGiveDumplings, BlondTakeDumplings),
        "npc/clerk_blond.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );
}

fn dialog_variants_handles(
    mut dialog_variant_source: ResMut<SelectedVariantsSource>,
    mut drevnira_state: ResMut<NextState<StrangeOldWoman>>,
    mut blond_state: ResMut<NextState<BlondAndGopniks>>,
    mut court_state: ResMut<NextState<Court>>,
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
                    court_state.set(Court::DrevniraStopped)
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&BLOND_FIRST_DIALOG);
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

    let selected = dialog_variant_source.consume(&BLOND_GIVE_DUMPLINGS_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_BLOND_GIVE_DUMPLINGS_COMPLETED {
                    blond_state.set(BlondAndGopniks::Completed);
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&BLOND_TAKE_DUMPLINGS_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_BLOND_TAKE_DUMPLINGS_JUST_COMPLETED {
                    blond_state.set(BlondAndGopniks::Completed);
                }
                if id == END_DIALOG_BLOND_TAKE_DUMPLINGS_NECK_TWISTED {
                    blond_state.set(BlondAndGopniks::Completed);
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&GUARDIAN_FIRST_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_GUARDIAN_FIRST_JUST_COMPLETED {
                    court_state.set(Court::Completed)
                }
                if id == END_DIALOG_GUARDIAN_FIRST_BEATEN {
                    court_state.set(Court::Completed)
                }
                if id == END_DIALOG_GUARDIAN_FIRST_DREVNIRA_STOP_ACCEPTED {
                    court_state.set(Court::StopDrevnira)
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&GUARDIAN_SECOND_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_GUARDIAN_SECOND_COMPLETED {
                    court_state.set(Court::Completed)
                }
                if id == END_DIALOG_GUARDIAN_SECOND_BEATEN {
                    court_state.set(Court::Completed)
                }
            }
        }
    }

    let selected = dialog_variant_source.consume(&GUARDIAN_THIRD_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == END_DIALOG_GUARDIAN_THIRD_COMPLETED {
                    court_state.set(Court::Completed)
                }
                if id == END_DIALOG_GUARDIAN_THIRD_BEATEN {
                    court_state.set(Court::Completed)
                }
            }
        }
    }
}

fn unload() {}