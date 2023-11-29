use bevy::{
    ecs::entity::Entity,
    prelude::{
        AssetServer, Assets, BuildChildren, Commands, Input, IntoSystemConfigs, KeyCode, Plugin,
        Query, Res, ResMut, Startup, Transform, Update, Vec2, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, GravityScale, LockedAxes, RigidBody, Velocity};

use crate::{
    core::{components::BodyYOffset, z_index::DEFAULT_OBJECT_Z},
    interaction::component::{ActiveInteractor, InteractionArea, InteractionSide},
};

use super::{
    components::{FightAnimation, MoveAnimation, Player},
    resources::PlayerAnimations,
    types::{FightDirection, MoveDirection},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, player_spawns)
            .add_systems(Update, player_fight)
            .add_systems(Update, player_movement.after(player_fight))
            .add_systems(Update, player_animation.after(player_movement))
            .add_systems(Update, basic_animation.after(player_animation))
            .add_systems(Update, basic_fight_animation.after(player_animation))
            .add_systems(Update, change_interaction_area.after(player_movement));
    }
}

fn player_spawns(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<PlayerAnimations>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let start_move_direction = MoveDirection::ForwardIdle;
    let start_fight_direction = FightDirection::Forward;

    let moves_handle = asset_server.load("npc/formidable_face.png");
    let move_atlas = TextureAtlas::from_grid(moves_handle, Vec2::new(32.0, 46.0), 6, 8, None, None);

    let fight_handle = asset_server.load("npc/formidable_face_fight.png");
    let fight_atlas =
        TextureAtlas::from_grid(fight_handle, Vec2::new(64.0, 68.0), 6, 4, None, None);

    let move_atlas_handle = texture_atlases.add(move_atlas);
    let fight_atlas_handle = texture_atlases.add(fight_atlas);

    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteSheetBundle {
            texture_atlas: move_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: character_animations.moves[&start_move_direction].0 as usize,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MoveAnimation {
            timer: Timer::from_seconds(
                character_animations.moves[&start_move_direction].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: start_move_direction,
            sheet_handle: move_atlas_handle,
        })
        .insert(FightAnimation {
            timer: Timer::from_seconds(
                character_animations.fight[&start_fight_direction].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: start_fight_direction,
            sheet_handle: fight_atlas_handle,
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
            is_fights: false,
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
        });
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in player_info.iter_mut() {
        if player.is_fights {
            velocity.linvel.x = 0.0;
            velocity.linvel.y = 0.0;
            return;
        }

        let up = keyboard.pressed(KeyCode::W);
        let down = keyboard.pressed(KeyCode::S);
        let left = keyboard.pressed(KeyCode::A);
        let right = keyboard.pressed(KeyCode::D);

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

fn player_fight(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    character_animations: Res<PlayerAnimations>,
    mut player_query: Query<(Entity, &MoveAnimation, &FightAnimation, &mut Player)>,
) {
    for (entity, move_animation, fight_animation, mut player) in player_query.iter_mut() {
        if keyboard.pressed(KeyCode::L) && keyboard.just_pressed(KeyCode::L) {
            let sprite_part = character_animations.fight[&fight_animation.direction];
            commands
                .entity(entity)
                .insert(fight_animation.sheet_handle.clone())
                .insert(TextureAtlasSprite {
                    index: sprite_part.0 as usize,
                    ..Default::default()
                });
            player.is_fights = true;
        } else if keyboard.just_released(KeyCode::L) {
            let sprite_part = character_animations.moves[&move_animation.direction];
            commands
                .entity(entity)
                .insert(move_animation.sheet_handle.clone())
                .insert(TextureAtlasSprite {
                    index: sprite_part.0 as usize,
                    ..Default::default()
                });
            player.is_fights = false;
        }
    }
}

fn player_animation(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<PlayerAnimations>,
    mut player_query: Query<
        (
            &mut MoveAnimation,
            &mut FightAnimation,
            &mut TextureAtlasSprite,
            &Velocity,
            &Player,
        ),
        With<Player>,
    >,
) {
    for (mut move_animation, mut fight_animation, mut sprite, rb_vels, player) in
        player_query.iter_mut()
    {
        let mut restart_animation = false;
        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
            if keyboard_input.just_released(KeyCode::A) {
                move_animation.direction = MoveDirection::LeftIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::D) {
                move_animation.direction = MoveDirection::RightIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::W) {
                move_animation.direction = MoveDirection::BackwardIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::S) {
                move_animation.direction = MoveDirection::ForwardIdle;
                restart_animation = true;
            }
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            move_animation.direction = MoveDirection::LeftMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::D) {
            move_animation.direction = MoveDirection::RightMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::W) {
            move_animation.direction = MoveDirection::BackwardMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            move_animation.direction = MoveDirection::ForwardMove;
            restart_animation = true;
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            fight_animation.direction = FightDirection::Left;
        } else if keyboard_input.just_pressed(KeyCode::D) {
            fight_animation.direction = FightDirection::Right;
        } else if keyboard_input.just_pressed(KeyCode::W) {
            fight_animation.direction = FightDirection::Backward;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            fight_animation.direction = FightDirection::Forward;
        }

        if restart_animation {
            if !player.is_fights {
                let sprite_part = character_animations.moves[&move_animation.direction];
                sprite.index = sprite_part.0 as usize;
                move_animation.timer =
                    Timer::from_seconds(sprite_part.2, bevy::time::TimerMode::Repeating);
            }

            let sprite_part = character_animations.fight[&fight_animation.direction];
            fight_animation.timer =
                Timer::from_seconds(sprite_part.2, bevy::time::TimerMode::Repeating);
        }
    }
}

fn change_interaction_area(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut ActiveInteractor, With<Player>>,
) {
    for mut interactor in player_query.iter_mut() {
        if keyboard_input.just_released(KeyCode::A) {
            interactor.area = InteractionArea::create(8.0, 20.0, -16.0, 0.0);
            interactor.side = InteractionSide::Left;
        } else if keyboard_input.just_released(KeyCode::D) {
            interactor.area = InteractionArea::create(8.0, 20.0, 16.0, 0.0);
            interactor.side = InteractionSide::Right;
        } else if keyboard_input.just_released(KeyCode::W) {
            interactor.area = InteractionArea::from_sizes(8.0, 20.0);
            interactor.side = InteractionSide::Top;
        } else if keyboard_input.just_released(KeyCode::S) {
            interactor.area = InteractionArea::from_sizes(8.0, 20.0);
            interactor.side = InteractionSide::Bottom;
        }
    }
}

fn basic_animation(
    time: Res<Time>,
    character_animations: Res<PlayerAnimations>,
    mut animation_query: Query<(&mut MoveAnimation, &mut TextureAtlasSprite, &Player)>,
) {
    for (mut move_animation, mut sprite, player) in animation_query.iter_mut() {
        if player.is_fights {
            return;
        }
        move_animation.timer.tick(time.delta());

        if move_animation.timer.finished() && !player.is_fights {
            let animation_idxs = character_animations.moves[&move_animation.direction];
            if sprite.index >= animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}

fn basic_fight_animation(
    time: Res<Time>,
    character_animations: Res<PlayerAnimations>,
    mut animation_query: Query<(&mut FightAnimation, &mut TextureAtlasSprite, &Player)>,
) {
    for (mut fight_animation, mut sprite, player) in animation_query.iter_mut() {
        if !player.is_fights {
            return;
        }
        fight_animation.timer.tick(time.delta());

        if fight_animation.timer.finished() {
            let animation_idxs = character_animations.fight[&fight_animation.direction];
            if sprite.index >= animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}
