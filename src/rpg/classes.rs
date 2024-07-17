use crate::rpg::{Agility, Charisma, Fortitude, NeckGnawing, NeckTwisting, Stamina, Strength};

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