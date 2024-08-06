use bevy::prelude::*;
use crate::gui::Container;

pub struct Button {
    bundle: ButtonBundle,
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
        }
    }
}

impl Button {
    pub fn size_percentage(&mut self, width: f32, height: f32) -> &mut Button {
        self.bundle.style.width = Val::Percent(width);
        self.bundle.style.height = Val::Percent(height);
        self
    }

    pub fn width_percentage(&mut self, width: f32) -> &mut Button {
        self.bundle.style.width = Val::Percent(width);
        self
    }

    pub fn margin(&mut self, margin: f32) -> &mut Button {
        self.bundle.style.margin = UiRect {
            left: Val::Px(margin),
            right: Val::Px(margin),
            top: Val::Px(margin),
            bottom: Val::Px(margin),
        };
        self
    }

    pub fn border_radius(&mut self, radius: f32) -> &mut Button {
        self.bundle.border_radius = BorderRadius {
            top_left: Val::Px(radius),
            top_right: Val::Px(radius),
            bottom_left: Val::Px(radius),
            bottom_right: Val::Px(radius),
        };
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

    pub fn spawn(
        self,
        parent: &mut ChildBuilder,
        payload: impl Bundle,
        spawn_children: impl FnOnce(&mut ChildBuilder),
    ) {
        parent
            .spawn(self.bundle)
            .with_children(spawn_children)
            .insert(payload);
    }
}
