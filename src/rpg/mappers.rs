use crate::gui::GetSelectorItem;
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack, Dumplings, Venison};

impl GetSelectorItem for DirectionalAttack {
    fn localised_name(&self) -> String {
        match self {
            DirectionalAttack::InevitableDamage { .. } => "Абсолютный урон".to_string(),
            DirectionalAttack::Punch { .. } => "Удар кулаком".to_string(),
            DirectionalAttack::Kick { .. } => "Удар ногой по харе".to_string(),
            DirectionalAttack::Bite { .. } => "Укус за пятку".to_string(),
            DirectionalAttack::PawStrike { .. } => "Удар задней лапой".to_string(),
        }
    }

    fn localised_description(&self) -> String {
        match self {
            DirectionalAttack::InevitableDamage { damage } => {
                format!("Ignores armor.\n\nDamage = {damage}")
            }
            DirectionalAttack::Punch { damage } => {
                format!("Ignores 5% armor.\n\nDamage = {damage}")
            }
            DirectionalAttack::Kick { damage } => {
                format!("Ignores 5% armor.\n\nDamage = {damage}")
            }
            DirectionalAttack::Bite { damage } => {
                format!("Ignores 5% armor.\n\nDamage = {damage}")
            }
            DirectionalAttack::PawStrike { damage } => {
                format!("Ignores 5% armor.\n\nDamage = {damage}")
            }
        }
    }
}

impl GetSelectorItem for Ability {
    fn localised_name(&self) -> String {
        match self {
            Ability::NeckTwist { .. } => "Щею свернуть!".to_string(),
            Ability::WoundsLicking { .. } => "Зализать раны".to_string(),
            Ability::SuperPunch { .. } => "Могучий удар кулаком".to_string(),
            Ability::NeckGnawing { .. } => "Разгрызть шею!".to_string(),
        }
    }

    fn localised_description(&self) -> String {
        match self {
            Ability::NeckTwist { damage, cost } => {
                format!("It has 3% to defeat enemy, else makes damage = {damage}.\n\nEnergy cost = {cost}.\n\nIgnores armor.")
            }
            Ability::WoundsLicking { health, cost } => {
                format!("No treatment? Lick your wound... Literally.\n\nRegain health = {health}.\n\nEnergy cost = {cost}.")
            }
            Ability::SuperPunch { .. } => "TODO".to_string(),
            Ability::NeckGnawing { .. } => "TODO".to_string(),
        }
    }
}

impl GetSelectorItem for ConsumableItem {
    fn localised_name(&self) -> String {
        match self {
            ConsumableItem::Dumplings(_) => "Dumplings".to_string(),
            ConsumableItem::Venison(_) => "Venison".to_string(),
        }
    }

    fn localised_description(&self) -> String {
        match self {
            ConsumableItem::Dumplings(Dumplings { health, energy }) => {
                format!("It's very tasty.\n\nRegain {health} of health and {energy} of energy.")
            }
            ConsumableItem::Venison(Venison { health, energy }) => {
                format!(
                    "It's quite tasty, but you will be lazy if eat it.\n\n\
                Regain {health} of health, but take energy = {energy}."
                )
            }
        }
    }
}
