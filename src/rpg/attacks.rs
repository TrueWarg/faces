use crate::rpg::{DirectionalAction, TargetProps};

struct InevitableDamage {
    damage: i32,
}

impl DirectionalAction for InevitableDamage {
    fn apply(&self, target: &mut TargetProps) {
        target.health.decrease(self.damage;
    }
}
