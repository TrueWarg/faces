use bevy::prelude::*;

use super::Text;

pub struct DynamicText {
    id: DynamicTextId,
    bundle: TextBundle,
}

#[derive(Component, Default)]
pub struct DynamicTextId {
    pub value: usize,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let section = TextSection {
            value: String::new(),
            style: TextStyle {
                font: Handle::default(),
                font_size: DynamicText::SIZE_MEDIUM,
                color: Color::WHITE,
            },
        };
        DynamicText {
            id: DynamicTextId::default(),
            bundle: TextBundle::from_sections(vec![section; 2])
                .with_text_justify(JustifyText::Center),
        }
    }
}

impl Text for DynamicText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn(self.bundle).insert(self.id);
    }
}

impl DynamicText {
    pub fn id(&mut self, id: usize) -> &mut DynamicText {
        self.id.value = id;
        self
    }

    pub fn dynamic_text_value<S: Into<String> + Clone>(&mut self, text: S) -> &mut DynamicText {
        self.bundle.text.sections[1].value = text.into();
        self
    }
}
