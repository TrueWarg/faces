use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{default, FlexDirection, NodeBundle, Style, Val};

use crate::gui::{Container, Scroll, ScrollableContent};

pub struct Scrollable {
    bundle: NodeBundle,
    scroll: Scroll,
}

impl Default for Scrollable {
    fn default() -> Scrollable {
        Scrollable {
            bundle: NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            scroll: Scroll::default(),
        }
    }
}

impl Scrollable {
    pub fn size_percentage(width: f32, height: f32) -> Scrollable {
        let mut scrollable = Self::default();
        scrollable.bundle.style.width = Val::Percent(width);
        scrollable.bundle.style.height = Val::Percent(height);
        scrollable
    }

    pub fn spawn(
        self,
        parent: &mut ChildBuilder,
        spawn_children: impl FnOnce(&mut ChildBuilder),
    )
    {
        parent
            .spawn((
                self.bundle,
                Scroll::default(),
            ))
            .with_children(|p| {
                p.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    ScrollableContent::default(),
                ))
                    .with_children(spawn_children);
            });
    }
}