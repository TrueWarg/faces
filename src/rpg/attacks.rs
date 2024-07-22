use crate::rpg::{DirectionalAction, TargetProps};

pub struct InevitableDamage {
    pub damage: i32,
}

pub struct Punch {
    pub damage: i32,
}

impl DirectionalAction for InevitableDamage {
    fn apply(&self, target: &mut TargetProps) {
        target.health.decrease(self.damage);
    }
}

impl DirectionalAction for Punch {
    fn apply(&self, target: &mut TargetProps) {
        target.health.decrease(self.damage);
    }
}
