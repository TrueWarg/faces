use crate::rpg::Agility;
use crate::rpg::Charisma;
use crate::rpg::Fortitude;
use crate::rpg::NeckGnawing;
use crate::rpg::NeckTwisting;
use crate::rpg::Stamina;
use crate::rpg::Strength;

pub enum Class {
    FormidableFace {
        strength: Strength,
        agility: Agility,
        stamina: Stamina,
        charisma: Charisma,
        neck_twisting: NeckTwisting,
        fortitude: Fortitude,
    },

    FormidableDog {
        strength: Strength,
        agility: Agility,
        stamina: Stamina,
        neck_gnawing: NeckGnawing,
    },
}