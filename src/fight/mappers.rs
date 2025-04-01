use crate::fight::{ActionTarget, GetActionTarget};
use crate::rpg::Ability;

impl GetActionTarget for Ability {
    fn action_target(&self) -> ActionTarget {
        return match self {
            Ability::NeckTwist { .. } => { ActionTarget::Enemy }
            Ability::WoundsLicking { .. } => { ActionTarget::Ally }
        };
    }
}
