use std::collections::HashMap;

use bevy::prelude::Resource;
use serde::Deserialize;

use super::types::MoveAnimationDirection;

#[derive(Resource, Deserialize)]
pub struct MoveAnimationResource {
    pub animations: HashMap<MoveAnimationDirection, (u32, u32, f32)>,
}


