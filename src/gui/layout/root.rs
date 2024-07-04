use bevy::prelude::*;

#[derive(Component)]
pub struct RootMarker;

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

    pub fn spawn(self, commands: &mut Commands, spawn_children: impl FnOnce(&mut ChildBuilder)) {
        commands
            .spawn(self.bundle)
            .with_children(spawn_children)
            .insert(RootMarker);
    }
}
