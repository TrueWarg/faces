use bevy::app::{Plugin, Update};
use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec3;
use bevy::prelude::{Commands, in_state, IntoSystemConfigs, OnEnter, OnExit, Query, Res, ResMut, SpriteBundle, States, TextureAtlasLayout, Transform};
use bevy_rapier2d::dynamics::RigidBody;
use crate::animation::entities::MoveDirection;
use crate::core::collisions::recalculate_z;
use crate::core::entities::LevelYMax;
use crate::core::z_index::{calculate_z, FLOOR_Z, ON_WALL_OBJECT_Z, WALL_Z};
use crate::level::objects::spawn_object;
use crate::npc::spawn_fixed_npc;

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
            .add_systems(Update, (recalculate_z).run_if(in_state(self.state.clone())));
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let y_max = LevelYMax::create(359.0);
    commands.spawn(y_max);

    spawn_floor(&mut commands, &asset_server);

    let wall_top = asset_server.load("courthouse_hall/wall_top.png");
    spawn_object(&mut commands, wall_top, 0.0, 359.0, WALL_Z, 256.0, 30.0, 0.0);

    spawn_doors(&mut commands, &asset_server, 0.0, 351.0);

    let wall_left = asset_server.load("courthouse_hall/wall_left.png");
    spawn_object(&mut commands, wall_left, -240.0, 30.0, WALL_Z, 17.0, 359.0, 0.0);

    let wall_right = asset_server.load("courthouse_hall/wall_right.png");
    spawn_object(&mut commands, wall_right, 240.0, 30.0, WALL_Z, 17.0, 359.0, 0.0);

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
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("courthouse_hall/floor.png"),
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

fn spawn_guardians(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_fixed_npc(
        &asset_server,
        &mut commands,
        &mut layouts,
        (),
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
        (),
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
        (),
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
        (),
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
        (),
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
        (),
        "npc/crazy_man.png".to_string(),
        MoveDirection::ForwardIdle,
        x,
        y,
        z,
    );
}

fn unload() {}