use crate::rpg::{DirectionalAction, TargetProps};

pub struct NeckTwist {
    pub damage: i32,
}

impl DirectionalAction for NeckTwist {
    fn apply(&self, target: &mut TargetProps) {
        target.health.decrease(target.health.max);
    }
}