use std::collections::HashMap;

use bevy::prelude::Resource;

use crate::animation::entities::MoveDirection;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub moves: HashMap<MoveDirection, (u32, u32, f32)>,
}

impl PlayerAnimations {
    pub fn default() -> PlayerAnimations {
        return PlayerAnimations {
            moves: Self::moves(),
        };
    }

    fn moves() -> HashMap<MoveDirection, (u32, u32, f32)> {
        let mut moves = HashMap::with_capacity(8);
        moves.insert(MoveDirection::ForwardIdle, (0, 5, 0.4));
        moves.insert(MoveDirection::LeftIdle, (6, 11, 0.4));
        moves.insert(MoveDirection::BackwardIdle, (12, 17, 0.4));
        moves.insert(MoveDirection::RightIdle, (18, 23, 0.4));
        moves.insert(MoveDirection::ForwardMove, (24, 29, 0.15));
        moves.insert(MoveDirection::LeftMove, (30, 35, 0.15));
        moves.insert(MoveDirection::BackwardMove, (36, 41, 0.15));
        moves.insert(MoveDirection::RightMove, (42, 47, 0.15));
        return moves;
    }
}
