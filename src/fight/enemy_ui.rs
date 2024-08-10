use bevy::color::palettes::css::{ANTIQUE_WHITE, GREEN};
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{Color, Component};

use crate::gui::{Button, Container};

pub struct EnemyItem {
    id: EnemyId,
}

#[derive(Component)]
pub struct EnemyId(pub usize);

impl EnemyItem {
    pub fn new(id: usize) -> EnemyItem {
        return EnemyItem { id: EnemyId(id) };
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        let mut border = Button::default();
        border
            .size_percentage(100.0, 100.0)
            .background_color(Color::from(ANTIQUE_WHITE));

        let mut frame = Container::size_percentage(90.0, 90.0);
        frame.justify_start();
        let mut avatar = Container::size_percentage(100.0, 80.0);
        avatar.background_color(Color::from(GREEN));
        border.spawn(parent, self.id, |parent| {
            frame.spawn(parent, |parent| {
                avatar.spawn_empty(parent);
            })
        });
    }
}
