use bevy::ecs::component::Component;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Handle, Image, SpriteBundle, Transform, TransformBundle};
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
