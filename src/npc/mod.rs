use bevy::app::{Plugin, Update};
use bevy::asset::{Assets, AssetServer};
use bevy::hierarchy::BuildChildren;
use bevy::math::UVec2;
use bevy::prelude::{Bundle, Commands, IntoSystemConfigs};
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
use crate::movement;
use crate::movement::entities::MoveAgent;
use crate::movement::routes::route_build;
pub use crate::npc::entities::{IdleAnimation, MoveAnimation, Npc};

mod entities;
mod animations;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, route_build)
            .add_systems(Update, idle_animation)
            .add_systems(Update, move_agent_moves.after(route_build))
            .add_systems(Update, npc_animation.after(move_agent_moves))
            .add_systems(Update, npc_basic_animation);
    }
}

pub fn spawn_fixed_npc(
    asset_server: &Res<AssetServer>,
    mut commands: &mut Commands,
    mut layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    marker: impl Bundle,
    asset_path: String,
    move_direction: MoveDirection,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
) {

    let moves_handle = asset_server.load(asset_path);
    let move_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 46), 6, 8, None, None,
    );

    let move_layout_handle = layouts.add(move_layout);

    let animations = NpcAnimations::default();
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
                animations.moves[&move_direction].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: move_direction,
            sheet_handle: move_layout_handle,
        })
        .insert(animations)
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::from(Transform::from_xyz(pos_x, pos_y, pos_z)))
        .insert(Npc { speed: 0.0, move_direction: movement::entities::MoveDirection::TopIdle })
        .insert(BodyYOffset::create(20.0))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(8.0, 4.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -16.0, DEFAULT_OBJECT_Z)));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(8.0, 20.0),
            side: InteractionSide::Bottom,
        });
}

pub fn spawn_formidable_dog(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    marker: impl Bundle,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    speed: f32,
    gravity_scale: f32,
) {
    let start_move_direction = MoveDirection::ForwardIdle;

    let moves_handle = asset_server.load("npc/formidable_dog.png");
    let move_layout = TextureAtlasLayout::from_grid(
        UVec2::new(24, 24), 4, 8, None, None,
    );

    let move_layout_handle = layouts.add(move_layout);

    let animations = NpcAnimations::dog();
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
            marker,
        ))
        .insert(MoveAnimation {
            timer: Timer::from_seconds(
                animations.moves[&start_move_direction].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: start_move_direction,
            sheet_handle: move_layout_handle,
        })
        .insert(animations)
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(gravity_scale))
        .insert(TransformBundle::from(Transform::from_xyz(pos_x, pos_y, pos_z)))
        .insert(Npc { speed, move_direction: movement::entities::MoveDirection::TopIdle })
        .insert(BodyYOffset::create(8.0))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(8.0, 4.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -8.0, DEFAULT_OBJECT_Z)));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(8.0, 20.0),
            side: InteractionSide::Bottom,
        })
        .insert(MoveAgent {
            priority: 1,
            half_size: 8,
            route: vec![],
        });
}

pub fn npc_animation(
    mut player_query: Query<(&mut MoveAnimation, &mut TextureAtlas, &Npc, &NpcAnimations, &Velocity)>,
) {
    for (mut move_animation, mut sprite, npc, npc_animation, rb_vels) in player_query.iter_mut() {
        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 && move_animation.direction.is_idle()
        {
            return;
        }
        let new_animation = match npc.move_direction {
            movement::entities::MoveDirection::Top => MoveDirection::BackwardMove,
            movement::entities::MoveDirection::LeftTop => MoveDirection::BackwardMove,
            movement::entities::MoveDirection::Left => MoveDirection::LeftMove,
            movement::entities::MoveDirection::LeftBottom => MoveDirection::ForwardMove,
            movement::entities::MoveDirection::Bottom => MoveDirection::ForwardMove,
            movement::entities::MoveDirection::RightBottom => MoveDirection::ForwardMove,
            movement::entities::MoveDirection::Right => MoveDirection::RightMove,
            movement::entities::MoveDirection::RightTop => MoveDirection::BackwardMove,
            movement::entities::MoveDirection::TopIdle => MoveDirection::BackwardIdle,
            movement::entities::MoveDirection::LeftTopIdle => MoveDirection::BackwardIdle,
            movement::entities::MoveDirection::LeftIdle => MoveDirection::LeftIdle,
            movement::entities::MoveDirection::LeftBottomIdle => MoveDirection::ForwardIdle,
            movement::entities::MoveDirection::BottomIdle => MoveDirection::ForwardIdle,
            movement::entities::MoveDirection::RightBottomIdle => MoveDirection::ForwardIdle,
            movement::entities::MoveDirection::RightIdle => MoveDirection::RightIdle,
            movement::entities::MoveDirection::RightTopIdle => MoveDirection::BackwardIdle,
        };
        let prev_animation = move_animation.direction != new_animation;
        move_animation.direction = new_animation;
        if prev_animation {
            let sprite_part = npc_animation.moves[&move_animation.direction];
            sprite.index = sprite_part.0 as usize;
            move_animation.timer =
                Timer::from_seconds(sprite_part.2, bevy::time::TimerMode::Repeating);
        }
    }
}

pub fn move_agent_moves(
    mut agents: Query<(&mut MoveAgent, &mut Npc, &mut Velocity, &Transform)>,
) {
    for (mut agent, mut npc, mut velocity, transform) in agents.iter_mut() {
        let route = &agent.route;
        if route.is_empty() {
            return;
        }
        let target = route.first().expect("Route musn't be empty");
        let translation = transform.translation;
        let x = translation.x;
        let y = translation.y;
        let target_x = target.x as f32;
        let target_y = target.y as f32;
        let eps = 24.0;
        if (x - target_x).abs() < eps && (y - target_y).abs() < eps {
            let route = &mut agent.route;
            route.pop();
            npc.move_direction = match npc.move_direction {
                movement::entities::MoveDirection::Top => movement::entities::MoveDirection::TopIdle,
                movement::entities::MoveDirection::LeftTop => movement::entities::MoveDirection::TopIdle,
                movement::entities::MoveDirection::Left => movement::entities::MoveDirection::LeftIdle,
                movement::entities::MoveDirection::LeftBottom => movement::entities::MoveDirection::LeftIdle,
                movement::entities::MoveDirection::Bottom => movement::entities::MoveDirection::BottomIdle,
                movement::entities::MoveDirection::RightBottom => movement::entities::MoveDirection::BottomIdle,
                movement::entities::MoveDirection::Right => movement::entities::MoveDirection::RightIdle,
                movement::entities::MoveDirection::RightTop => movement::entities::MoveDirection::RightIdle,
                _ => movement::entities::MoveDirection::TopIdle,
            };
            velocity.linvel.x = 0.0;
            velocity.linvel.y = 0.0;
        } else {
            let mut x_sign = -1;
            if (target_x - x) > 0.0 {
                x_sign = 1;
            }
            if (target_x - x).abs() < eps {
                x_sign = 0;
            }
            let mut y_sign = -1;
            if (target_y - y) > 0.0 {
                y_sign = 1;
            }
            if (target_y - y).abs() < eps {
                y_sign = 0;
            }
            velocity.linvel.x = npc.speed * x_sign as f32;
            velocity.linvel.y = npc.speed * y_sign as f32;
            if x_sign == 0 && y_sign > 1 {
                npc.move_direction = movement::entities::MoveDirection::Top;
            }
            if x_sign == -1 && y_sign > 1 {
                npc.move_direction = movement::entities::MoveDirection::LeftTop;
            }
            if x_sign == -1 && y_sign == 0 {
                npc.move_direction = movement::entities::MoveDirection::Left;
            }
            if x_sign == -1 && y_sign == -1 {
                npc.move_direction = movement::entities::MoveDirection::LeftBottom;
            }
            if x_sign == 0 && y_sign == -1 {
                npc.move_direction = movement::entities::MoveDirection::Bottom;
            }
            if x_sign == 1 && y_sign == -1 {
                npc.move_direction = movement::entities::MoveDirection::RightBottom;
            }
            if x_sign == 1 && y_sign == 0 {
                npc.move_direction = movement::entities::MoveDirection::Right;
            }
            if x_sign == 1 && y_sign == 1 {
                npc.move_direction = movement::entities::MoveDirection::RightTop;
            }
        }
    }
}

pub fn npc_basic_animation(
    time: Res<Time>,
    mut animation_query: Query<(&mut MoveAnimation, &mut TextureAtlas, &NpcAnimations)>,
) {
    for (mut move_animation, mut sprite, character_animations) in animation_query.iter_mut() {
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

fn idle_animation(
    time: Res<Time>,
    mut animation_query: Query<(&mut IdleAnimation, &mut TextureAtlas)>,
) {
    for (mut idle_animation, mut sprite) in animation_query.iter_mut() {
        idle_animation.timer.tick(time.delta());
        if idle_animation.timer.finished() {
            if sprite.index >= idle_animation.frames_count - 1 {
                sprite.index = 0;
            } else {
                sprite.index += 1;
            }
        }
    }
}
