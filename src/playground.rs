use bevy::prelude::{
    App, AssetServer, Assets, Camera2dBundle, Commands, Component, Handle, Image, Input, KeyCode,
    Name, OnEnter, Plugin, Query, Res, ResMut, Resource, Transform, Vec2, Vec3, With, Without,
};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::time::{Time, Timer};
use bevy::transform::TransformBundle;
use bevy::utils::HashMap;
use bevy_rapier2d::na::{Rotation, Rotation2};
use bevy_rapier2d::prelude::{
    Collider, GravityScale, KinematicCharacterController, LockedAxes, RigidBody, Velocity,
};
use ron::de::from_bytes;
use serde::Deserialize;

use crate::resources::AssetsPack;
use crate::states::GameState;

pub struct PlayGroundPlugin;

impl Plugin for PlayGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Exporation), build_map)
            .add_systems(OnEnter(GameState::Exporation), spawn_map);
    }
}

fn build_map(mut commands: Commands) {}

fn spawn_map(mut commands: Commands) {}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("wall_left.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::cuboid(128.0, 1.0));

    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: asset_server.load("wall_left.png"),
            transform: Transform {
                translation: Vec3::new(128.0, 0.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::cuboid(128.0, 1.0));

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("bottom_floor.png"),
        transform: Transform {
            translation: Vec3::new(0.0, -88.0, 15.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(RigidBody::Fixed).insert(SpriteBundle {
        texture: asset_server.load("bottom_floor.png"),
        transform: Transform {
            translation: Vec3::new(128.0, -88.0, 15.0),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
}

pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in player_info.iter_mut() {
        let up = keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up);
        let down = keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down);
        let left = keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left);
        let right = keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 {
            velocity.linvel.x = player.speed * (x_axis as f32);
        } else {
            velocity.linvel.x = 0.0;
        }

        if y_axis != 0 {
            velocity.linvel.y = player.speed * (y_axis as f32);
        } else {
            velocity.linvel.y = 0.0;
        }
    }
}

pub fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<CharacterAnimationResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let character_starting_animation = CharacterAnimationType::ForwardIdle;
    let texture_handle = asset_server.load("player_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: character_animations.animations[&character_starting_animation].0 as usize,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CharacterAnimationComponent {
            timer: Timer::from_seconds(
                character_animations.animations[&character_starting_animation].2,
                bevy::time::TimerMode::Repeating,
            ),
            animation_type: character_starting_animation.clone(),
        })
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            60.0, -100.0, 25.0,
        )))
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(Player { speed: 100.0 });
}

pub fn spawn_nps(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteBundle {
            texture: asset_server.load("npc.png"),
            ..Default::default()
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, -100.0, 25.0,
        )))
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(Npc { speed: 0.0 });
}

pub fn set_player_animation_system(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<CharacterAnimationResource>,
    mut player_query: Query<
        (
            &mut CharacterAnimationComponent,
            &mut TextureAtlasSprite,
            &Velocity,
        ),
        With<Player>,
    >,
) {
    for (mut character_animation, mut sprite, rb_vels) in player_query.iter_mut() {
        let mut restart_animation = false;

        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
            if keyboard_input.just_released(KeyCode::A)
                || keyboard_input.just_released(KeyCode::Left)
            {
                character_animation.animation_type = CharacterAnimationType::LeftIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::D)
                || keyboard_input.just_released(KeyCode::Right)
            {
                character_animation.animation_type = CharacterAnimationType::RightIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::W)
                || keyboard_input.just_released(KeyCode::Up)
            {
                character_animation.animation_type = CharacterAnimationType::BackwardIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::S)
                || keyboard_input.just_released(KeyCode::Down)
            {
                character_animation.animation_type = CharacterAnimationType::ForwardIdle;
                restart_animation = true;
            }
        }
        if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left) {
            character_animation.animation_type = CharacterAnimationType::LeftMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::D)
            || keyboard_input.just_pressed(KeyCode::Right)
        {
            character_animation.animation_type = CharacterAnimationType::RightMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::W)
            || keyboard_input.just_pressed(KeyCode::Up)
        {
            character_animation.animation_type = CharacterAnimationType::BackwardMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::S)
            || keyboard_input.just_pressed(KeyCode::Down)
        {
            character_animation.animation_type = CharacterAnimationType::ForwardMove;
            restart_animation = true;
        }

        if restart_animation {
            let animation_data =
                character_animations.animations[&character_animation.animation_type];
            sprite.index = animation_data.0 as usize;
            character_animation.timer =
                Timer::from_seconds(animation_data.2, bevy::time::TimerMode::Repeating);
        }
    }
}

pub fn animate_character_system(
    time: Res<Time>,
    character_animations: Res<CharacterAnimationResource>,
    mut animation_query: Query<(&mut CharacterAnimationComponent, &mut TextureAtlasSprite)>,
) {
    for (mut character_animation, mut sprite) in animation_query.iter_mut() {
        character_animation.timer.tick(time.delta());

        if character_animation.timer.finished() {
            let animation_idxs =
                character_animations.animations[&character_animation.animation_type];
            if sprite.index == animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct BasicAnimationComponent;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Debug)]
pub struct Player {
    speed: f32,
}
#[derive(Component, Debug)]
pub struct Npc {
    speed: f32,
}

#[derive(Resource, Deserialize)]
pub struct CharacterAnimationResource {
    // start and end indexes of animations
    pub animations: HashMap<CharacterAnimationType, (u32, u32, f32)>,
}

#[derive(Hash, PartialEq, Eq, Clone, Deserialize)]
pub enum CharacterAnimationType {
    ForwardIdle,
    LeftIdle,
    BackwardIdle,
    RightIdle,
    ForwardMove,
    LeftMove,
    BackwardMove,
    RightMove,
}

impl CharacterAnimationType {
    fn is_idle(&self) -> bool {
        matches!(
            self,
            CharacterAnimationType::ForwardIdle
                | CharacterAnimationType::BackwardIdle
                | CharacterAnimationType::LeftIdle
                | CharacterAnimationType::RightIdle
        )
    }
}

#[derive(Component)]
pub struct CharacterAnimationComponent {
    pub timer: Timer,
    pub animation_type: CharacterAnimationType,
}
