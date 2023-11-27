use bevy::{
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
    components::{MoveAnimationComponent, Player},
    resources::MoveAnimationResource,
    types::MoveAnimationDirection,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, player_spawns)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_move_animation.after(player_movement))
            .add_systems(Update, basic_animation.after(player_move_animation))
            .add_systems(Update, change_interaction_area.after(player_movement));
    }
}

fn player_spawns(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    character_animations: Res<MoveAnimationResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let character_starting_animation = MoveAnimationDirection::ForwardIdle;
    let texture_handle = asset_server.load("npc/formidable_face.png");
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
        .insert(MoveAnimationComponent {
            timer: Timer::from_seconds(
                character_animations.animations[&character_starting_animation].2,
                bevy::time::TimerMode::Repeating,
            ),
            direction: character_starting_animation.clone(),
        })
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            60.0,
            -100.0,
            DEFAULT_OBJECT_Z,
        )))
        .insert(Player { speed: 200.0 })
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

fn player_move_animation(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<MoveAnimationResource>,
    mut player_query: Query<
        (
            &mut MoveAnimationComponent,
            &mut TextureAtlasSprite,
            &Velocity,
        ),
        With<Player>,
    >,
) {
    for (mut character_animation, mut sprite, rb_vels) in player_query.iter_mut() {
        let mut restart_animation = false;

        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
            if keyboard_input.just_released(KeyCode::A) {
                character_animation.direction = MoveAnimationDirection::LeftIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::D) {
                character_animation.direction = MoveAnimationDirection::RightIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::W) {
                character_animation.direction = MoveAnimationDirection::BackwardIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::S) {
                character_animation.direction = MoveAnimationDirection::ForwardIdle;
                restart_animation = true;
            }
        }
        if keyboard_input.just_pressed(KeyCode::A) {
            character_animation.direction = MoveAnimationDirection::LeftMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::D) {
            character_animation.direction = MoveAnimationDirection::RightMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::W) {
            character_animation.direction = MoveAnimationDirection::BackwardMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            character_animation.direction = MoveAnimationDirection::ForwardMove;
            restart_animation = true;
        }

        if restart_animation {
            let animation_data = character_animations.animations[&character_animation.direction];
            sprite.index = animation_data.0 as usize;
            character_animation.timer =
                Timer::from_seconds(animation_data.2, bevy::time::TimerMode::Repeating);
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
    character_animations: Res<MoveAnimationResource>,
    mut animation_query: Query<(&mut MoveAnimationComponent, &mut TextureAtlasSprite)>,
) {
    for (mut character_animation, mut sprite) in animation_query.iter_mut() {
        character_animation.timer.tick(time.delta());

        if character_animation.timer.finished() {
            let animation_idxs = character_animations.animations[&character_animation.direction];
            if sprite.index == animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}
