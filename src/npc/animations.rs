use std::collections::HashMap;

use bevy::prelude::Component;

use crate::animation::entities::MoveDirection;

#[derive(Component)]
pub struct NpcAnimations {
    pub moves: HashMap<MoveDirection, (u32, u32, f32)>,
}

impl NpcAnimations {
    pub fn default() -> NpcAnimations {
        NpcAnimations {
            moves: Self::default_modes(),
        }
    }

    pub fn dog() -> NpcAnimations {
        NpcAnimations {
            moves: Self::dog_moves(),
        }
    }

    fn default_modes() -> HashMap<MoveDirection, (u32, u32, f32)> {
        let mut moves = HashMap::with_capacity(8);
        moves.insert(MoveDirection::ForwardIdle, (0, 5, 0.4));
        moves.insert(MoveDirection::LeftIdle, (6, 11, 0.4));
        moves.insert(MoveDirection::BackwardIdle, (12, 17, 0.4));
        moves.insert(MoveDirection::RightIdle, (18, 23, 0.4));
        moves.insert(MoveDirection::ForwardMove, (24, 29, 0.15));
        moves.insert(MoveDirection::LeftMove, (30, 35, 0.15));
        moves.insert(MoveDirection::BackwardMove, (36, 41, 0.15));
        moves.insert(MoveDirection::RightMove, (42, 47, 0.15));
        moves
    }

    fn dog_moves() -> HashMap<MoveDirection, (u32, u32, f32)> {
        let mut moves = HashMap::with_capacity(8);
        moves.insert(MoveDirection::ForwardIdle, (0, 3, 0.6));
        moves.insert(MoveDirection::LeftIdle, (4, 7, 0.6));
        moves.insert(MoveDirection::BackwardIdle, (8, 11, 0.6));
        moves.insert(MoveDirection::RightIdle, (12, 15, 0.6));
        moves.insert(MoveDirection::ForwardMove, (16, 19, 0.10));
        moves.insert(MoveDirection::LeftMove, (20, 23, 0.10));
        moves.insert(MoveDirection::BackwardMove, (24, 27, 0.10));
        moves.insert(MoveDirection::RightMove, (28, 31, 0.10));
        moves
    }
}
