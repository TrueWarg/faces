use crate::core::entities::Description;
use crate::core::state_machines::FiniteLinearTransition;
use crate::interaction::interactors::{
    detect_active_interaction, ActiveInteractor, Container, ContainerState, InteractionArea,
    InteractionSide, LimitedInteractor, PassiveInteractor,
};
use crate::level::sprites::WoodenChestSprites;
use crate::party::PartyStateStorage;
use crate::rpg::ConsumableItem;
use crate::sound::ChestSounds;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::ecs::component::Component;
use bevy::hierarchy::BuildChildren;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Changed, Commands, Entity, Handle, Image, KeyCode, Query, Res, ResMut, SpriteBundle, Transform, TransformBundle, With};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

#[derive(Component)]
pub struct WoodenChest;

#[derive(Component)]
pub struct LevelArm;

pub fn spawn_object(
    commands: &mut Commands,
    image: Handle<Image>,
    x: f32,
    y: f32,
    z: f32,
    half_x: f32,
    half_y: f32,
    collider_shift_y: f32,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(half_x, half_y))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    collider_shift_y,
                    z,
                )));
        });
}

pub fn spawn_container(
    commands: &mut Commands,
    marker: impl Bundle,
    image: Handle<Image>,
    x: f32,
    y: f32,
    z: f32,
    collider_shift_y: f32,
    items: Vec<ConsumableItem>,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert((
            SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker,
        ))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(16.0, 11.0))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    collider_shift_y,
                    z,
                )));
        })
        .insert(PassiveInteractor {
            area: InteractionArea::from_sizes(16.0, 11.0),
            side: InteractionSide::Bottom,
        })
        .insert(LimitedInteractor)
        .insert(WoodenChest)
        .insert(Container::<ConsumableItem> {
            state: ContainerState::initial_state(),
            items,
        })
        .insert(Description {
            text: "Closed chest".to_string(),
        });
}

pub fn wooden_chest_states_draws<T: Bundle + Clone>(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    sprites: Res<WoodenChestSprites>,
    chests: Query<(Entity, &Container<T>), Changed<Container<T>>>,
    audio_res: Res<ChestSounds>,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }
    for (entity, container) in chests.iter() {
        let new_sprite = match container.state {
            ContainerState::Closed => sprites.closed.clone(),
            ContainerState::Full => {
                commands.spawn(AudioBundle {
                    source: audio_res.opened.clone(),
                    settings: PlaybackSettings::ONCE,
                });
                sprites.full.clone()
            }
            ContainerState::Empty => {
                commands.spawn(AudioBundle {
                    source: audio_res.items_picked.clone(),
                    settings: PlaybackSettings::ONCE,
                });
                sprites.empty.clone()
            }
        };
        commands.entity(entity).insert(new_sprite);
    }
}

pub fn interact_with_container_handle(
    mut commands: Commands,
    mut party_state_storage: ResMut<PartyStateStorage>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    mut interactors: Query<
        (
            Entity,
            &PassiveInteractor,
            &Transform,
            &mut Container<ConsumableItem>,
        ),
        With<LimitedInteractor>,
    >,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }
    for (entity, interactor, transform, mut container) in interactors.iter_mut() {
        let is_interacting = detect_active_interaction(&active, (interactor, transform));
        if is_interacting {
            container.state = container.state.transit();
            if container.state.is_finished() {
                party_state_storage.add_consumables(container.items.clone());
                commands.entity(entity).remove::<LimitedInteractor>();
            }
        }
    }
}
