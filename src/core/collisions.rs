use bevy::prelude::{Query, Transform};
use bevy_rapier2d::prelude::RigidBody;

use super::{
    components::{BodyYOffset, LevelYMax},
    z_index::calculate_z,
};

pub fn recalculate_z(
    level_y_max: Query<&LevelYMax>,
    mut bodies: Query<(&mut Transform, &RigidBody, &BodyYOffset)>,
) {
    let y_max = level_y_max.get_single().expect("no level max found");
    for (mut transform, body, y_offset) in bodies.iter_mut() {
        match body {
            RigidBody::Dynamic => {
                let new_z = calculate_z(transform.translation.y - y_offset.value, y_max.value);
                transform.translation.z = new_z;
            }
            _ => {}
        }
    }
}
