use bevy::asset::Handle;
use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::GRAY;
use bevy::prelude::{AlignContent, BackgroundColor, Changed, Commands, Component, Font, Interaction, Query, With};

use crate::gui::{Button, Container, InScroll, Root, Scrollable, Text};

pub struct Selector;

#[derive(Component)]
pub struct SelectorItem {
    pub name: String,
    pub description: String,
}

#[derive(Component)]
pub struct PosAndDescr(pub usize, pub String);

#[derive(Component)]
pub struct SelectedItemPosHolder {
    value: Option<usize>,
}

#[derive(Component)]
pub struct Description;

impl SelectedItemPosHolder {
    pub fn new() -> Self {
        return SelectedItemPosHolder { value: None };
    }

    fn store(&mut self, value: usize) {
        self.value = Some(value);
    }

    pub fn take_away_unsafe(&mut self) -> usize {
        let value = self.value.expect("Value was not be stored");
        self.value = None;
        return value;
    }

    pub fn take_away(&mut self) -> Option<usize> {
        let value = self.value;
        self.value = None;
        return value;
    }
}

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
            .background_color(Color::from(Srgba::new(0.302, 0.302, 0.302, 0.7)));
        let mut description_container = Container::size_percentage(50.0, 100.0);
        let mut scrollable = Scrollable::size_percentage(50.0, 100.0);
        root.spawn(commands, marker, |parent| {
            main_container.spawn(parent, |parent| {
                description_container.spawn(parent, |parent| {
                    let mut text = Text::medium("", font);
                    text = text.justify_start();
                    text.size_percentage(95.0, 95.0);
                    text.spawn_with_payload(parent, Description);
                });
                scrollable.spawn(parent, |parent| {
                    for (pos, item) in items.iter().enumerate() {
                        let mut wrapper = Container::auto();
                        wrapper.spawn(parent, |parent| {
                            let mut button = Button::default();
                            button
                                .width_percentage(100.0)
                                .height(100.0);
                            button.spawn(parent, (InScroll, PosAndDescr(pos, item.description.clone())), |parent| {
                                let mut text = Text::medium(&item.name, font);
                                text = text.justify_start();
                                text
                                    .width_percentage(100.0)
                                    .margin(25.0);
                                text.spawn(parent);
                            });
                        });
                    }
                });
            })
        });
    }
}

pub fn pick_item_handle<S>(
    mut query: Query<
        (&PosAndDescr, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PosAndDescr>),
    >,
    mut description_query: Query<(&mut bevy::prelude::Text), With<Description>>,
    mut holder_query: Query<(&mut SelectedItemPosHolder)>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = Color::NONE.into();
            }
            Interaction::Hovered => {
                let mut description = description_query.single_mut();
                description.sections[0].value = item.1.clone();
                *background_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                let mut holder = holder_query.single_mut();
                holder.store(item.0);
            }
        }
    }
}

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_BUTTON_COLOR: Color = Color::srgba(0.50, 0.50, 0.50, 0.7);