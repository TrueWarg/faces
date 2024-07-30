use bevy::asset::Handle;
use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::{BLUE, GRAY, RED};
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, Component, default, Font, JustifyText, NodeBundle, Style, TextBundle, TextStyle, UiRect, Val};

use crate::gui::{Button, Container, InScroll, Root, Scrollable, Text};

pub struct Selector;

#[derive(Component)]
pub struct SelectorItem {
    pub name: String,
    pub description: String,
}

#[derive(Component)]
pub struct SelectedItemPos(pub usize);

impl Selector {
    pub fn spawn(
        self,
        commands: &mut Commands,
        marker: impl Component,
        font: &Handle<Font>,
        items: Vec<SelectorItem>,
    ) {
        let mut root = Root::default();
        let mut main_container = Container::default();
        main_container
            .row()
            .justify_start()
            .background_color(Color::from(Srgba::new(0.502, 0.502, 0.502, 0.5)));
        let mut description = Container::size_percentage(50.0, 100.0);
        description.background_color(Color::from(GRAY));
        let mut scrollable = Scrollable::size_percentage(50.0, 100.0);
        root.spawn(commands, marker, |parent| {
            main_container.spawn(parent, |parent| {
                description.spawn(parent, |parent| {});
                scrollable.spawn(parent, |parent| {
                    for (pos, item) in items.iter().enumerate() {
                        let mut wrapper = Container::auto();
                        wrapper.margin(10.0);
                        wrapper.spawn(parent, |parent| {
                            let mut button = Button::default();
                            button
                                .width(600.0)
                                .height(100.0);
                            button.spawn(parent, (InScroll, SelectedItemPos(pos)), |parent| {
                                let mut name = Container::default();
                                name.row().justify_start();
                                name.spawn(parent, |parent| {
                                    let text = Text::medium(&item.name, font);
                                    text.spawn(parent)
                                });
                            });
                        });
                    }
                });
            })
        });
    }
}