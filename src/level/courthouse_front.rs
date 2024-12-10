use bevy::app::{Plugin, Update};
use bevy::asset::{Assets, AssetServer};
use bevy::hierarchy::BuildChildren;
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{Commands, in_state, IntoSystemConfigs, Query, TextureAtlas, Timer, TransformBundle};
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
use crate::core::z_index::{calculate_z, DEFAULT_OBJECT_Z, FLOOR_Z, MIN_RANGE_Z, ON_WALL_OBJECT_Z, WALL_Z};
use crate::interaction::interactors::{InteractionArea, InteractionSide, PassiveInteractor};
use crate::npc::IdleAnimation;

pub struct CourtHouseFrontPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for CourtHouseFrontPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(OnEnter(self.state.clone()), spawn_old_woman_drevnira)
            .add_systems(OnExit(GameState::Exploration), unload)
            .add_systems(Update, recalculate_z.run_if(in_state(self.state.clone())));
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
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

fn unload() {}