use std::collections::HashMap;

use bevy::prelude::Resource;

use crate::animation::entities::{MoveDirection, FightDirection};

#[derive(Resource)]
pub struct PlayerAnimations {
    pub moves: HashMap<MoveDirection, (u32, u32, f32)>,
    pub fight: HashMap<FightDirection, (u32, u32, f32)>,
}

impl PlayerAnimations {
    pub fn default() -> PlayerAnimations {
        return PlayerAnimations {
            moves: Self::moves(),
            fight: Self::fight(),
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

    fn fight() -> HashMap<FightDirection, (u32, u32, f32)> {
        let mut fight = HashMap::with_capacity(4);
        fight.insert(FightDirection::Forward, (0, 5, 0.1));
        fight.insert(FightDirection::Left, (6, 11, 0.1));
        fight.insert(FightDirection::Backward, (12, 17, 0.1));
        fight.insert(FightDirection::Right, (18, 23, 0.1));
        return fight;
    }
}
