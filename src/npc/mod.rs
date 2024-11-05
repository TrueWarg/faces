use bevy::asset::{Assets, AssetServer};
use bevy::hierarchy::BuildChildren;
use bevy::input::ButtonInput;
use bevy::math::UVec2;
use bevy::prelude::{Bundle, Commands};
use bevy::prelude::KeyCode;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::SpriteBundle;
use bevy::prelude::TextureAtlas;
use bevy::prelude::TextureAtlasLayout;
use bevy::prelude::Time;
use bevy::prelude::Timer;
use bevy::prelude::Transform;
use bevy::prelude::TransformBundle;
use bevy::prelude::With;
use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::dynamics::LockedAxes;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::geometry::Collider;

pub use animations::NpcAnimations;

use crate::animation::entities::MoveDirection;
use crate::core::entities::BodyYOffset;
use crate::core::z_index::DEFAULT_OBJECT_Z;
use crate::interaction::interactors::{InteractionArea, InteractionSide, PassiveInteractor};
use crate::movement::entities::Target;
pub use crate::npc::entities::{MoveAnimation, Npc, IdleAnimation};

mod entities;
mod animations;

pub fn spawns_npc(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<NpcAnimations>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    marker: impl Bundle,
    asset_path: String,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    speed: f32,
    gravity_scale: f32,
) {
    let start_move_direction = MoveDirection::ForwardIdle;

    let moves_handle = asset_server.load(asset_path);
    let move_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 46), 6, 8, None, None,
    );

    let move_layout_handle = layouts.add(move_layout);

    commands
        .spawn(RigidBody::Fixed)
        .insert((
            SpriteBundle {
                texture: moves_handle,
                ..Default::default()
            },
            TextureAtlas {
                layout: move_layout_handle.clone(),
                index: 0,
            },
            marker,
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
        .insert(GravityScale(gravity_scale))
        .insert(TransformBundle::from(Transform::from_xyz(
            pos_x,
            pos_y,
            pos_z,
        )))
        .insert(Npc { speed })
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
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(8.0, 20.0),
            side: InteractionSide::Bottom,
        })
        .insert(Target { half_size: 16 });
}

pub fn npc_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    character_animations: Res<NpcAnimations>,
    mut npc_query: Query<
        (
            &mut MoveAnimation,
            &mut TextureAtlas,
            &Velocity,
        ),
        With<Npc>,
    >,
) {
    for (mut move_animation, mut sprite, rb_vels) in
        npc_query.iter_mut()
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

pub fn npc_basic_animation(
    time: Res<Time>,
    character_animations: Res<NpcAnimations>,
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
