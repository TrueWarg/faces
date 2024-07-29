use bevy::asset::Handle;
use bevy::color::Color;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{Component, Font};

use crate::gui::{Button, Text};

pub struct ActionItem {
    id: ActionId,
    button: Button,
    text: Text,
}

#[derive(Component)]
pub struct ActionId(pub usize);

impl ActionItem {
    pub fn new<S: Into<String> + Clone>(id: ActionId, value: S, font: &Handle<Font>) -> Self {
        return ActionItem {
            id,
            button: Button::default(),
            text: Text::medium(value, font),
        };
    }

    pub fn size_percentage(&mut self, width: f32, height: f32) -> &mut Self {
        self.button.size_percentage(width, height);
        self
    }

    pub fn margin(&mut self, margin: f32) -> &mut Self {
        self.button.margin(margin);
        self
    }

    pub fn text_color(&mut self, color: Color) -> &mut Self {
        self.text.set_color(color);
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        self.button.spawn(parent, self.id, |parent| {
            self.text.spawn(parent);
        })
    }
}