use crate::gui::GetSelectorItem;
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack, Dumplings, Venison};

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