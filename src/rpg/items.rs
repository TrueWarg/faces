use crate::rpg::{DirectionalAction, TargetProps};

pub struct Dumplings {
    pub health: i32,
    pub energy: i32,
}

impl DirectionalAction for Dumplings {
    fn apply(&self, target: &mut TargetProps) {
        target.health.increase(self.health);
        target.energy.increase(self.health);
    }
}

pub struct Venison {
    pub health: i32,
    pub energy: i32,
}

impl DirectionalAction for Venison {
    fn apply(&self, target: &mut TargetProps) {
        target.health.increase(self.health);
        target.energy.decrease(self.health);
    }
}