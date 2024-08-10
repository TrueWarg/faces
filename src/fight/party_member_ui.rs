use bevy::color::palettes::basic::{GRAY, MAROON};
use bevy::color::palettes::css::{ANTIQUE_WHITE, GREEN, YELLOW};
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{Color, Component, Font, Handle};

use crate::gui::{Button, Container};

pub struct PartyMemberItem {
    id: MemberId,
}

#[derive(Component)]
pub struct MemberId(pub usize);

#[derive(Component)]
pub struct Health;

#[derive(Component)]
pub struct Energy;

impl PartyMemberItem {
    pub fn new(id: usize) -> PartyMemberItem {
        return PartyMemberItem { id: MemberId(id)};
    }

    pub fn spawn(self, parent: &mut ChildBuilder, font: &Handle<Font>) {
        let mut border = Button::default();
        border
            .size_percentage(100.0, 100.0)
            .background_color(Color::from(ANTIQUE_WHITE));

        let mut frame = Container::size_percentage(90.0, 90.0);
        frame.justify_start();
        let mut avatar = Container::size_percentage(100.0, 80.0);
        avatar.background_color(Color::from(MAROON));
        let mut health_status = Container::size_percentage(100.0, 10.0);
        health_status.background_color(Color::from(GREEN));
        let mut energy_status = Container::size_percentage(100.0, 10.0);
        energy_status.background_color(Color::from(YELLOW));
        border.spawn(parent, self.id, |parent| {
            frame.spawn(parent, |parent| {
                avatar.spawn_empty(parent);
                health_status.spawn_with_payload(parent, Health, |parent| {});
                energy_status.spawn_with_payload(parent, Energy, |parent| {});
            })
        });
    }
}
