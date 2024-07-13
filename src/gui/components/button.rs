use bevy::prelude::*;

use super::text::{SimpleText, Text};

pub struct Button {
    id: ButtonId,
    bundle: ButtonBundle,
    text: SimpleText,
}

#[derive(Component, Default, PartialEq, Copy, Clone)]
pub struct ButtonId {
    pub value: usize,
}

impl Default for Button {
    fn default() -> Button {
        let style = Style {
            width: Val::Px(400.0),
            height: Val::Px(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        Button {
            bundle: ButtonBundle {
                background_color: Color::NONE.into(),
                style,
                ..default()
            },
            text: SimpleText::default(),
            id: ButtonId::default(),
        }
    }
}

impl Button {
    pub fn new<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Button {
        Button {
            text: SimpleText::medium(value, font),
            ..default()
        }
    }

    pub fn square<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Button {
        let mut button = Self::default();
        button.bundle.style.width = Val::Px(60.0);
        button.bundle.style.height = Val::Px(60.0);
        button.text = SimpleText::medium(value, font);
        button
    }

    pub fn text_color(&mut self, color: Color) -> &mut Button {
        self.text.color(color);
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Button {
        self.bundle.background_color = color.into();
        self
    }

    pub fn width(&mut self, width: f32) -> &mut Button {
        self.bundle.style.width = Val::Px(width);
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Button {
        self.bundle.style.height = Val::Px(height);
        self
    }

    pub fn id(&mut self, id: ButtonId) -> &mut Button {
        self.id = id;
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent
            .spawn(self.bundle)
            .with_children(|parent| self.text.spawn(parent))
            .insert(self.id);
    }
}
