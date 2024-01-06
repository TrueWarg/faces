use bevy::{ecs::system::Query, math::Vec3, transform::components::Transform};
use bevy_rapier2d::{dynamics::Velocity, na::ComplexField};

use crate::movement::component::MoveAgent;

use super::component::Npc;

pub fn move_melee_fighters(
    mut agents: Query<(&mut MoveAgent, &mut Npc, &mut Velocity, &Transform)>,
) {
    for (mut agent, npc, velocity, transform) in agents.iter_mut() {
        let route = &agent.route;
        if route.is_empty() {
            return;
        }
        let target = route.first().expect("Route musn't be empty");
        let translation = transform.translation;
        let x = translation.x;
        let y = translation.y;
        let eps = (agent.square.half_size as f32) / 4.0;
        if (x - target.x as f32).abs() < eps && (y - target.x as f32).abs() < eps {
            let route = &mut agent.route;
            route.pop();
        } else {
            
        }
    }
}
