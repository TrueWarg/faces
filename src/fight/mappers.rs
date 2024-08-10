use crate::fight::ActionTarget;
use crate::fight::selector_ui::SelectorItem;
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack, Dumplings, Venison};

pub trait GetSelectorItem {
    fn selector_item(&self) -> SelectorItem {
        return SelectorItem {
            name: self.localised_name(),
            description: self.localised_description(),
        };
    }
    fn localised_name(&self) -> String;
    fn localised_description(&self) -> String;
}

pub trait GetActionTarget {
    fn action_target(&self) -> ActionTarget;
}

impl GetSelectorItem for DirectionalAttack {
    fn localised_name(&self) -> String {
        return match self {
            DirectionalAttack::InevitableDamage { .. } => { "Absolute damage".to_string() }
            DirectionalAttack::Punch { .. } => { "Punch".to_string() }
        };
    }

    fn localised_description(&self) -> String {
        return match self {
            DirectionalAttack::InevitableDamage { damage } => { format!("Ignores armor.\n\nDamage = {damage}") }
            DirectionalAttack::Punch { damage } => { format!("Ignores 5% armor.\n\nDamage = {damage}") }
        };
    }
}

impl GetSelectorItem for Ability {
    fn localised_name(&self) -> String {
        return match self {
            Ability::NeckTwist { .. } => { "Neck twist!".to_string() }
            Ability::WoundsLicking { .. } => { "Wounds licking".to_string() }
        };
    }

    fn localised_description(&self) -> String {
        return match self {
            Ability::NeckTwist { damage, cost } => {
                format!("It has 3% to defeat enemy, else makes damage = {damage}.\n\nEnergy cost = {cost}.\n\nIgnores armor.")
            }
            Ability::WoundsLicking { health, cost } => {
                format!("No treatment? Lick your wound... Literally.\n\nRegain health = {health}.\n\nEnergy cost = {cost}.")
            }
        };
    }
}

impl GetActionTarget for Ability {
    fn action_target(&self) -> ActionTarget {
        return match self {
            Ability::NeckTwist { .. } => { ActionTarget::Enemy }
            Ability::WoundsLicking { .. } => { ActionTarget::Ally }
        };
    }
}

impl GetSelectorItem for ConsumableItem {
    fn localised_name(&self) -> String {
        return match self {
            ConsumableItem::Dumplings(_) => { "Dumplings".to_string() }
            ConsumableItem::Venison(_) => { "Venison".to_string() }
        };
    }

    fn localised_description(&self) -> String {
        return match self {
            ConsumableItem::Dumplings(Dumplings { health, energy }) => {
                format!("It's very tasty.\n\nRegain {health} of health and {energy} of energy.")
            }
            ConsumableItem::Venison(Venison { health, energy }) => {
                format!("It's quite tasty, but you will be lazy if eat it.\n\n\
                Regain {health} of health, but take energy = {energy}.")
            }
        };
    }
}