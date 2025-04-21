use bevy::{
    prelude::Assets,
    prelude::AssetServer,
    prelude::BuildChildren,
    prelude::ButtonInput,
    prelude::Commands,
    prelude::IntoSystemConfigs,
    prelude::KeyCode,
    prelude::Plugin,
    prelude::Query,
    prelude::Res,
    prelude::ResMut,
    prelude::Startup,
    prelude::Transform,
    prelude::Update,
    prelude::With,
    sprite::{TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer},
};
use bevy::math::UVec2;
use bevy::prelude::{in_state, OnEnter, SystemParamFunction, TransformBundle, Without};
use bevy::sprite::SpriteBundle;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::GravityScale;
use bevy_rapier2d::prelude::LockedAxes;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Velocity;

use crate::{
    animation::entities::MoveDirection,
    core::{entities::BodyYOffset, z_index::DEFAULT_OBJECT_Z},
    interaction::interactors::{ActiveInteractor, InteractionArea, InteractionSide},
    movement::entities::Target,
};
use crate::core::entities::MainCamera;
use crate::core::states::GameState;

use super::{
    animations::PlayerAnimations,
    entities::{MoveAnimation, Player},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Exploration)))
            .add_systems(Update, camera_movement.after(player_movement).run_if(in_state(GameState::Exploration)))
            .add_systems(Update, player_animation.after(player_movement).run_if(in_state(GameState::Exploration)))
            .add_systems(Update, basic_animation.after(player_animation).run_if(in_state(GameState::Exploration)))
            .add_systems(Update, change_interaction_area.after(player_movement).run_if(in_state(GameState::Exploration)));
    }
}

fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<PlayerAnimations>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let start_move_direction = MoveDirection::ForwardIdle;

    let moves_handle = asset_server.load("npc/formidable_face.png");
    let move_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 46), 6, 8, None, None);

    let move_layout_handle = layouts.add(move_layout);

    commands
        .spawn(RigidBody::Dynamic)
        .insert((
            SpriteBundle {
                texture: moves_handle,
                ..Default::default()
            },
            TextureAtlas {
                layout: move_layout_handle.clone(),
                index: 0,
            },
        ))
        .insert(MoveAnimation {
            timer: Timer::from_seconds(
                character_animations.moves[&start_move_direction].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: start_move_direction,
            sheet_handle: move_layout_handle,
        })
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            60.0,
            -100.0,
            DEFAULT_OBJECT_Z,
        )))
        .insert(Player {
            speed: 200.0,
        })
        .insert(BodyYOffset::create(20.0))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(8.0, 4.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    -16.0,
                    DEFAULT_OBJECT_Z,
                )));
        })
        .insert(ActiveInteractor {
            area: InteractionArea::from_sizes(8.0, 20.0),
            side: InteractionSide::Bottom,
        })
        .insert(Target { half_size: 16 });
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in player_info.iter_mut() {
        let up = keyboard.pressed(KeyCode::KeyW);
        let down = keyboard.pressed(KeyCode::KeyS);
        let left = keyboard.pressed(KeyCode::KeyA);
        let right = keyboard.pressed(KeyCode::KeyD);

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

pub fn camera_movement(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    character_animations: Res<PlayerAnimations>,
    mut player_query: Query<
        (
            &mut MoveAnimation,
            &mut TextureAtlas,
            &Velocity,
        ),
        With<Player>,
    >,
) {
    for (mut move_animation, mut sprite, rb_vels) in
        player_query.iter_mut()
    {
        let mut restart_animation = false;
        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
            if keyboard_input.just_released(KeyCode::KeyA) {
                move_animation.direction = MoveDirection::LeftIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::KeyD) {
                move_animation.direction = MoveDirection::RightIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::KeyW) {
                move_animation.direction = MoveDirection::BackwardIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::KeyS) {
                move_animation.direction = MoveDirection::ForwardIdle;
                restart_animation = true;
            }
        }

        if keyboard_input.just_pressed(KeyCode::KeyA) {
            move_animation.direction = MoveDirection::LeftMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::KeyD) {
            move_animation.direction = MoveDirection::RightMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::KeyW) {
            move_animation.direction = MoveDirection::BackwardMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::KeyS) {
            move_animation.direction = MoveDirection::ForwardMove;
            restart_animation = true;
        }

        if restart_animation {
            let sprite_part = character_animations.moves[&move_animation.direction];
            sprite.index = sprite_part.0 as usize;
            move_animation.timer =
                Timer::from_seconds(sprite_part.2, bevy::time::TimerMode::Repeating);
        }
    }
}

fn change_interaction_area(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut ActiveInteractor, With<Player>>,
) {
    for mut interactor in player_query.iter_mut() {
        if keyboard_input.just_released(KeyCode::KeyA) {
            interactor.area = InteractionArea::create(8.0, 20.0, -16.0, 0.0);
            interactor.side = InteractionSide::Left;
        } else if keyboard_input.just_released(KeyCode::KeyD) {
            interactor.area = InteractionArea::create(8.0, 20.0, 16.0, 0.0);
            interactor.side = InteractionSide::Right;
        } else if keyboard_input.just_released(KeyCode::KeyW) {
            interactor.area = InteractionArea::from_sizes(8.0, 20.0);
            interactor.side = InteractionSide::Top;
        } else if keyboard_input.just_released(KeyCode::KeyS) {
            interactor.area = InteractionArea::from_sizes(8.0, 20.0);
            interactor.side = InteractionSide::Bottom;
        }
    }
}

fn basic_animation(
    time: Res<Time>,
    character_animations: Res<PlayerAnimations>,
    mut animation_query: Query<(&mut MoveAnimation, &mut TextureAtlas)>,
) {
    for (mut move_animation, mut sprite) in animation_query.iter_mut() {
        move_animation.timer.tick(time.delta());

        if move_animation.timer.finished() {
            let animation_idxs = character_animations.moves[&move_animation.direction];
            if sprite.index >= animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}
