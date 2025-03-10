use std::cmp::{max, min};
use bevy::prelude::Component;

#[derive(Component, Clone, PartialEq, Debug)]
pub struct TargetProps {
    pub health: RangedProp,
    pub energy: RangedProp,
    pub armor: i32,
    pub evasion: i32,
}

impl TargetProps {
    pub fn is_defeated(&self) -> bool {
        return self.health.current <= self.health.min;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RangedProp {
    pub min: i32,
    pub current: i32,
    pub max: i32,
}

impl RangedProp {
    pub fn decrease(&mut self, value: i32) -> bool {
        let prev = self.current;
        self.current = max(self.min, self.current - value);
        return prev != self.current
    }

    pub fn increase(&mut self, value: i32) -> bool{
        let prev = self.current;
        self.current = min(self.max, self.current + value);
        return prev != self.current
    }
}
