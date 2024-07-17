use bevy::prelude::*;

pub struct Root {
    bundle: NodeBundle,
}

impl Default for Root {
    fn default() -> Root {
        Root {
            bundle: NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
        }
    }
}

impl Root {
    pub fn background_color(&mut self, color: Color) -> &mut Root {
        self.bundle.background_color = color.into();
        self
    }

    pub fn justify_between(&mut self) -> &mut Root {
        self.bundle.style.justify_content = JustifyContent::SpaceBetween;
        self
    }

    pub fn justify_around(&mut self) -> &mut Root {
        self.bundle.style.justify_content = JustifyContent::SpaceAround;
        self
    }

    pub fn spawn(
        self, commands:
        &mut Commands,
        marker: impl Component,
        spawn_children: impl FnOnce(&mut ChildBuilder),
    ) {
        commands
            .spawn(self.bundle)
            .with_children(spawn_children)
            .insert(marker);
    }
}
