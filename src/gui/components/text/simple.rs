use bevy::prelude::*;

use super::Text;

pub struct SimpleText {
    bundle: TextBundle,
}

impl Default for SimpleText {
    fn default() -> SimpleText {
        let style = TextStyle {
            font: Handle::default(),
            font_size: SimpleText::SIZE_MEDIUM,
            color: Color::WHITE,
        };
        SimpleText {
            bundle: TextBundle::from_section(String::new(), style)
                .with_text_justify(JustifyText::Center),
        }
    }
}

impl Text for SimpleText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn(self.bundle);
    }

    fn spawn_with_payload(self, parent: &mut ChildBuilder, payload: impl Bundle) {
        parent.spawn(self.bundle).insert(payload);
    }
}
