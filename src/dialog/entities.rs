use bevy::prelude::Component;
use crate::dialog::graph::DialogStick;

#[derive(Component)]
pub struct Dialog {
    pub id: DialogId,
    pub label: Option<String>,
    pub root: DialogStick,
}

#[derive(Component)]
pub struct DialogId(pub usize);