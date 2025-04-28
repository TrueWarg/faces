use crate::world_state::{GoIntoCourt, Trial};
use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;

use crate::world_state::quests::BlondAndGopniks;
use crate::world_state::quests::Court;
use crate::world_state::quests::EnterTheCourt;
use crate::world_state::quests::EscapeFromHouse;
use crate::world_state::quests::House;
use crate::world_state::quests::InCourHall;
use crate::world_state::quests::StrangeOldWoman;

pub struct WorldStatePlugin;

impl Plugin for WorldStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EscapeFromHouse>()
            .init_state::<EnterTheCourt>()
            .init_state::<InCourHall>()
            .init_state::<Court>()
            .init_state::<House>()
            .init_state::<StrangeOldWoman>()
            .init_state::<BlondAndGopniks>()
            .init_state::<GoIntoCourt>()
            .init_state::<Trial>();
    }
}
