use bevy::app::Update;
use bevy::prelude::{Commands, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, States};
use crate::core::collisions::recalculate_z;

pub struct FightingScene<S: States> {
    pub state: S,
}

impl<S: States> Plugin for FightingScene<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(self.state.clone()), load)
            .add_systems(OnExit(self.state.clone()), unload)
            .add_systems(
                Update,
                (recalculate_z).run_if(in_state(self.state.clone())),
            );
    }
}

fn load(mut commands: Commands) {

}

fn unload() {}