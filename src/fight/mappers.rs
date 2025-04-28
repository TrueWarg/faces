use crate::fight::{ActionTarget, GetActionTarget};
use crate::rpg::Ability;

impl GetActionTarget for Ability {
    fn action_target(&self) -> ActionTarget {
        match self {
            Ability::NeckTwist { .. } => ActionTarget::Enemy,
            Ability::WoundsLicking { .. } => ActionTarget::Ally,
            Ability::SuperPunch { .. } => ActionTarget::Enemy,
            Ability::NeckGnawing { .. } => ActionTarget::Enemy,
        }
    }
}
