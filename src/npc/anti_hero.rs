use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    math::Vec2,
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer},
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier2d::{
    dynamics::{AdditionalMassProperties, GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::Collider,
};

use crate::{
    animation::{self},
    core::{components::BodyYOffset, z_index::DEFAULT_OBJECT_Z},
    movement::component::{MoveAgent, MoveDirection},
    player::{components::MoveAnimation, resources::PlayerAnimations},
};

use super::component::Npc;

pub fn spawn_anti_hero(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<PlayerAnimations>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let start_move_direction = animation::entities::MoveDirection::ForwardIdle;
    // let start_fight_direction = FightDirection::Forward;

    let moves_handle = asset_server.load("npc/formidable_face.png");
    let move_layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 46.0), 6, 8, None, None);

    // let fight_handle = asset_server.load("npc/formidable_face_fight.png");
    // let fight_atlas = TextureAtlasLayout::from_grid(Vec2::new(64.0, 68.0), 6, 4, None, None);

    let move_layout_handle = layouts.add(move_layout);
    // let fight_layout_handle = layouts.add(fight_atlas);

    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: move_layout_handle.clone(),
                index: 0,
            },
            texture: moves_handle,
            ..Default::default()
        })
        .insert(MoveAnimation {
            timer: Timer::from_seconds(
                character_animations.moves[&start_move_direction].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: start_move_direction,
            sheet_handle: move_layout_handle,
        })
        // .insert(FightAnimation {
        //     timer: Timer::from_seconds(
        //         character_animations.fight[&start_fight_direction].2,
        //         bevy::time::TimerMode::Repeating,
        //     ),
        //     direction: start_fight_direction,
        //     sheet_handle: fight_layout_handle,
        // })
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.0))
        .insert(AdditionalMassProperties::Mass(100_000.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -60.0,
            -50.0,
            DEFAULT_OBJECT_Z,
        )))
        .insert(Npc {
            speed: 50.0,
            move_direction: MoveDirection::TopIdle,
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
        .insert(MoveAgent {
            priority: 1,
            half_size: 16,
            route: vec![],
        });
}

pub fn anti_hero_animation(
    character_animations: Res<PlayerAnimations>,
    mut player_query: Query<(&mut MoveAnimation, &mut TextureAtlas, &Npc, &Velocity)>,
) {
    for (mut move_animation, mut sprite, npc, rb_vels) in player_query.iter_mut() {
        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 && move_animation.direction.is_idle()
        {
            return;
        }
        let new_animation = match npc.move_direction {
            MoveDirection::Top => animation::entities::MoveDirection::BackwardMove,
            MoveDirection::LeftTop => animation::entities::MoveDirection::BackwardMove,
            MoveDirection::Left => animation::entities::MoveDirection::LeftMove,
            MoveDirection::LeftBottom => animation::entities::MoveDirection::ForwardMove,
            MoveDirection::Bottom => animation::entities::MoveDirection::ForwardMove,
            MoveDirection::RightBottom => animation::entities::MoveDirection::ForwardMove,
            MoveDirection::Right => animation::entities::MoveDirection::RightMove,
            MoveDirection::RightTop => animation::entities::MoveDirection::BackwardMove,
            MoveDirection::TopIdle => animation::entities::MoveDirection::BackwardIdle,
            MoveDirection::LeftTopIdle => animation::entities::MoveDirection::BackwardIdle,
            MoveDirection::LeftIdle => animation::entities::MoveDirection::LeftIdle,
            MoveDirection::LeftBottomIdle => animation::entities::MoveDirection::ForwardIdle,
            MoveDirection::BottomIdle => animation::entities::MoveDirection::ForwardIdle,
            MoveDirection::RightBottomIdle => animation::entities::MoveDirection::ForwardIdle,
            MoveDirection::RightIdle => animation::entities::MoveDirection::RightIdle,
            MoveDirection::RightTopIdle => animation::entities::MoveDirection::BackwardIdle,
        };

        let prev_animation = move_animation.direction != new_animation;
        move_animation.direction = new_animation;
        if prev_animation {
            let sprite_part = character_animations.moves[&move_animation.direction];
            sprite.index = sprite_part.0 as usize;
            move_animation.timer =
                Timer::from_seconds(sprite_part.2, bevy::time::TimerMode::Repeating);
        }
    }
}

pub fn basic_animation(
    time: Res<Time>,
    character_animations: Res<PlayerAnimations>,
    mut animation_query: Query<(&mut MoveAnimation, &mut TextureAtlas), With<Npc>>,
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
