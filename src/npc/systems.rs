use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{schedule::IntoSystemConfigs, system::Query},
    transform::components::Transform,
};
use bevy_rapier2d::dynamics::Velocity;

use crate::movement::{
    component::{MoveAgent, MoveDirection},
    systems::route_build,
};

use super::anti_hero::{anti_hero_animation, basic_animation, spawn_anti_hero};
use super::component::Npc;

pub struct MainNpcPlugin;

impl Plugin for MainNpcPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_anti_hero)
            .add_systems(Update, route_build)
            .add_systems(Update, move_melee_fighters.after(route_build))
            .add_systems(Update, anti_hero_animation.after(move_melee_fighters))
            .add_systems(Update, basic_animation.after(anti_hero_animation));
    }
}

pub fn move_melee_fighters(
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
        let eps = (agent.half_size as f32) / 4.0;
        if (x - target_x).abs() < eps && (y - target_x).abs() < eps {
            let route = &mut agent.route;
            route.pop();
            npc.move_direction = match npc.move_direction {
                MoveDirection::Top => MoveDirection::Top,
                MoveDirection::LeftTop => MoveDirection::LeftTop,
                MoveDirection::Left => MoveDirection::Left,
                MoveDirection::LeftBottom => MoveDirection::LeftBottom,
                MoveDirection::Bottom => MoveDirection::Bottom,
                MoveDirection::RightBottom => MoveDirection::RightBottom,
                MoveDirection::Right => MoveDirection::Right,
                MoveDirection::RightTop => MoveDirection::RightTop,
                _ => MoveDirection::TopIdle,
            }
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
                npc.move_direction = MoveDirection::Top;
            }

            if x_sign == -1 && y_sign > 1 {
                npc.move_direction = MoveDirection::LeftTop;
            }

            if x_sign == -1 && y_sign == 0 {
                npc.move_direction = MoveDirection::Left;
            }

            if x_sign == -1 && y_sign == -1 {
                npc.move_direction = MoveDirection::LeftBottom;
            }

            if x_sign == 0 && y_sign == -1 {
                npc.move_direction = MoveDirection::Bottom;
            }

            if x_sign == 1 && y_sign == -1 {
                npc.move_direction = MoveDirection::RightBottom;
            }

            if x_sign == 1 && y_sign == 0 {
                npc.move_direction = MoveDirection::Right;
            }

            if x_sign == 1 && y_sign == 1 {
                npc.move_direction = MoveDirection::RightTop;
            }
        }
    }
}
