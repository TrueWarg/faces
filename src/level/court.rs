use bevy::app::{Plugin, Update};
use bevy::asset::{Assets, AssetServer};
use bevy::hierarchy::BuildChildren;
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{Commands, Component, NextState};
use bevy::prelude::in_state;
use bevy::prelude::IntoSystemConfigs;
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

use crate::core::collisions::recalculate_z;
use crate::core::entities::{BodyYOffset, LevelYMax};
use crate::core::z_index::{calculate_z, DEFAULT_OBJECT_Z, FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z};
use crate::dialog::SelectedVariantsSource;
use crate::interaction::interactors::{InteractionArea, InteractionSide, PassiveInteractor};
use crate::level::{dialog_starts, JUDGES_FIRST_DIALOG_BEATEN, JUDGES_FIRST_DIALOG_COMPLETED};
use crate::level::HasDialogId;
use crate::level::JUDGES_FIRST_DIALOG;
use crate::level::JUDGES_SECOND_DIALOG;
use crate::level::JUDGES_THIRD_DIALOG;
use crate::level::objects::spawn_object;
use crate::npc::IdleAnimation;
use crate::world_state::Trial;

#[derive(Component)]
struct JudgesDialog;

impl HasDialogId for JudgesDialog {
    fn dialog_id(&self) -> usize {
        return JUDGES_FIRST_DIALOG;
    }
}

#[derive(Component)]
struct JudgesFormidableFaceWon;

impl HasDialogId for JudgesFormidableFaceWon {
    fn dialog_id(&self) -> usize {
        return JUDGES_SECOND_DIALOG;
    }
}

#[derive(Component)]
struct JudgesFormidableFaceFailed;

impl HasDialogId for JudgesFormidableFaceFailed {
    fn dialog_id(&self) -> usize {
        return JUDGES_THIRD_DIALOG;
    }
}

pub struct CourtPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for CourtPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnEnter(self.state.clone()), spawn_judges)
            .add_systems(Update, dialog_starts::<JudgesDialog>.run_if(in_state(Trial::SpeakWithJudges)))
            .add_systems(Update, dialog_starts::<JudgesFormidableFaceWon>.run_if(in_state(Trial::FormidableFaceWon)))
            .add_systems(Update, dialog_starts::<JudgesFormidableFaceFailed>.run_if(in_state(Trial::FormidableFaceFailed)))
            .add_systems(Update, (dialog_variants_handles, recalculate_z).run_if(in_state(self.state.clone())));
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut trial_state: ResMut<NextState<Trial>>,
) {
    trial_state.set(Trial::SpeakWithJudges);
    let y_max = LevelYMax::create(226.0);
    commands.spawn(y_max);

    spawn_floor(&mut commands, &asset_server);

    let wall = asset_server.load("court/wall.png");
    spawn_object(&mut commands, wall, 0.0, y_max.value, WALL_Z, 359.0, 30.0, 0.0);

    spawn_doors(&mut commands, &asset_server, -200.0, 218.0);
    spawn_doors(&mut commands, &asset_server, 0.0, 218.0);
    spawn_doors(&mut commands, &asset_server, 200.0, 218.0);

    // -------------------------------------------------------------------
    let flower = asset_server.load("court/bench.png");
    let x = -270.0;
    let y = -60.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = -120.0;
    let y = -60.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = -270.0;
    let y = -125.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = -120.0;
    let y = -125.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = -270.0;
    let y = -190.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = -120.0;
    let y = -190.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = 270.0;
    let y = -60.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = 120.0;
    let y = -60.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = 270.0;
    let y = -125.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = 120.0;
    let y = -125.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = 270.0;
    let y = -190.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);

    let flower = asset_server.load("court/bench.png");
    let x = 120.0;
    let y = -190.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 65.0, 18.0, -5.0);
    // -------------------------------------------------------------------

    let flower = asset_server.load("courthouse_hall/work_place.png");
    let x = 280.0;
    let y = 180.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 15.0, 37.0, -5.0);

    let flower = asset_server.load("court/cage.png");
    let x = -297.0;
    let y = 169.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, flower, x, y, z, 64.0, 87.0, -5.0);

    // -------------------------------------------------------------------
    let tribune = asset_server.load("court/tribune.png");
    let x = -110.0;
    let y = 30.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tribune, x, y, z, 22.0, 23.0, -5.0);

    let tribune = asset_server.load("court/tribune.png");
    let x = 90.0;
    let y = 30.0;
    let z = calculate_z(y, y_max.value);
    spawn_object(&mut commands, tribune, x, y, z, 22.0, 23.0, -5.0);
    // -------------------------------------------------------------------
}

fn spawn_floor(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("court/floor.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, FLOOR_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_doors(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_hall/doors.png"),
            transform: Transform {
                translation: Vec3::new(x, y, ON_WALL_OBJECT_Z),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn spawn_judges(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_y_max: Query<&LevelYMax>,
) {
    let default = LevelYMax::create(226.0);
    let y_max = level_y_max.get_single().unwrap_or(&default);
    let x = 0.0;
    let y = 150.0;
    let z = calculate_z(y, y_max.value);

    let image_handle = asset_server.load("npc/judges.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(229, 108), 22, 1, None, None,
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
        ))
        .insert((JudgesDialog, JudgesFormidableFaceWon, JudgesFormidableFaceFailed))
        .insert(IdleAnimation {
            timer: Timer::from_seconds(
                0.15,
                bevy::time::TimerMode::Repeating,
            ),
            frames_count: 22,
        })
        .insert(TransformBundle::from(Transform::from_xyz(x, y, z)))
        .insert(BodyYOffset::create(30.0))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(115.0, 59.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    -8.0,
                    DEFAULT_OBJECT_Z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(229.0, 54.0),
            side: InteractionSide::Bottom,
        });
}

fn dialog_variants_handles(
    mut dialog_variant_source: ResMut<SelectedVariantsSource>,
    mut trial_state: ResMut<NextState<Trial>>,
) {
    // todo: it calculates on each frame.
    // make it when dialog_variant_source updates only.
    let selected = dialog_variant_source.consume(&JUDGES_FIRST_DIALOG);
    match selected {
        None => {}
        Some(ids) => {
            for id in ids {
                if id == JUDGES_FIRST_DIALOG_COMPLETED {
                    trial_state.set(Trial::Wait);
                }

                if id == JUDGES_FIRST_DIALOG_BEATEN {
                    trial_state.set(Trial::GoAtHome);
                }
            }
        }
    }
}

fn unload() {}