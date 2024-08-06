use bevy::asset::Handle;
use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::{DIM_GREY, GRAY};
use bevy::prelude::{BackgroundColor, Changed, Commands, Component, Font, Interaction, NextState, Query, ResMut, With};
use crate::gui::{Button, Container, InScroll, Root, Scrollable, Text};

pub struct Selector;

#[derive(Component)]
pub struct SelectorItem {
    pub name: String,
    pub description: String,
}

#[derive(Component)]
pub struct ItemPos(pub usize);

#[derive(Component)]
pub struct SelectedItemPosHolder {
    value: Option<usize>,
}

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
                        wrapper.spawn(parent, |parent| {
                            let mut button = Button::default();
                            button
                                .width_percentage(100.0)
                                .height(100.0);
                            button.spawn(parent, (InScroll, ItemPos(pos)), |parent| {
                                let mut name = Container::default();
                                name.margin(10.0);
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

pub fn pick_item_handle<S>(
    mut query: Query<
        (&ItemPos, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ItemPos>),
    >,
    mut holder_query: Query<(&mut SelectedItemPosHolder)>,
) {
    for (button_id, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = Color::NONE.into();
            }
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                let mut holder = holder_query.single_mut();
                holder.store(button_id.0);
            }
        }
    }
}

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_BUTTON_COLOR: Color = Color::srgba(0.50, 0.50, 0.50, 0.7);