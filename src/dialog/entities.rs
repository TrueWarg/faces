use bevy::prelude::Component;
use crate::dialog::graph::DialogStick;

#[derive(Component)]
pub struct Dialog {
    pub id: usize,
    pub label: Option<String>,
    pub root: DialogStick,
}