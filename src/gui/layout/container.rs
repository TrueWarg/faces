use bevy::prelude::*;

pub struct Container {
    pub bundle: NodeBundle,
}

impl Default for Container {
    fn default() -> Container {
        Container {
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

impl Container {
    pub fn size(width: f32, height: f32) -> Container {
        let mut container = Self::default();
        container.bundle.style.width = Val::Px(width);
        container.bundle.style.height = Val::Px(height);
        container
    }

    pub fn size_percentage(width: f32, height: f32) -> Container {
        let mut container = Self::default();
        container.bundle.style.width = Val::Percent(width);
        container.bundle.style.height = Val::Percent(height);
        container
    }

    pub fn auto() -> Container {
        let mut container = Self::default();
        container.bundle.style.width = Val::Auto;
        container.bundle.style.height = Val::Auto;
        container
    }

    pub fn absolute(&mut self) -> &mut Container {
        self.bundle.style.position_type = PositionType::Absolute;
        self
    }

    pub fn row(&mut self) -> &mut Container {
        self.bundle.style.flex_direction = FlexDirection::Row;
        self
    }

    pub fn justify_between(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::SpaceBetween;
        self
    }

    pub fn justify_around(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::SpaceAround;
        self
    }

    pub fn justify_start(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::FlexStart;
        self
    }

    pub fn justify_end(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::FlexEnd;
        self
    }

    pub fn items_start(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexStart;
        self
    }

    pub fn content_start(&mut self) -> &mut Container {
        self.bundle.style.align_content = AlignContent::FlexStart;
        self
    }

    pub fn content_end(&mut self) -> &mut Container {
        self.bundle.style.align_content = AlignContent::FlexEnd;
        self
    }

    pub fn wrap(&mut self) -> &mut Container {
        self.bundle.style.flex_wrap = FlexWrap::Wrap;
        self
    }

    pub fn align_start(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexStart;
        self
    }

    pub fn align_end(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexEnd;
        self
    }

    pub fn bottom_margin(&mut self, margin: f32) -> &mut Container {
        self.bundle.style.margin.bottom = Val::Px(margin);
        self
    }

    pub fn margin(&mut self, margin: f32) -> &mut Container {
        self.bundle.style.margin = UiRect {
            left: Val::Px(margin),
            right: Val::Px(margin),
            top: Val::Px(margin),
            bottom: Val::Px(margin),
        };
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Container {
        self.bundle.background_color = color.into();
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, spawn_children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn(self.bundle).with_children(spawn_children);
    }

    pub fn spawn_empty(self, parent: &mut ChildBuilder) {
        self.spawn(parent, |_| {});
    }
}
