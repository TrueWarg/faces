use bevy::asset::Handle;
use bevy::color::Color;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{Bundle, Font, JustifyText, TextBundle, TextSection, TextStyle, UiRect, Val};

pub struct Text {
    bundle: TextBundle,
}

impl Default for Text {
    fn default() -> Text {
        let section = TextSection {
            value: String::new(),
            style: TextStyle {
                font: Handle::default(),
                font_size: Text::SIZE_MEDIUM,
                color: Color::WHITE,
            },
        };
        Text {
            bundle: TextBundle::from_sections(vec![section; 1])
                .with_text_justify(JustifyText::Center),
        }
    }
}

impl Text {
    pub fn new<S: Into<String> + Clone>(value: S, font: &Handle<Font>, font_size: f32) -> Self {
        let mut text = Self::default();
        text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = font_size;
        });
        text
    }

    pub fn for_each_section(&mut self, f: impl FnMut(&mut TextSection)) {
        self.bundle.text.sections.iter_mut().for_each(f);
    }

    pub fn small<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        return Self::new(value, font, Self::SIZE_SMALL);
    }

    pub fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        return Self::new(value, font, Self::SIZE_MEDIUM);
    }

    pub fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        return Self::new(value, font, Self::SIZE_LARGE);
    }

    pub fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        return Self::new(value, font, Self::SIZE_EXTRA_LARGE);
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.for_each_section(|section| section.style.color = color);
        self
    }

    pub fn size_percentage(&mut self, width: f32, height: f32) -> &mut Self {
        self.bundle.style.width = Val::Percent(width);
        self.bundle.style.height = Val::Percent(height);
        self
    }

    pub fn width_percentage(&mut self, width: f32) -> &mut Self {
        self.bundle.style.width = Val::Percent(width);
        self
    }

    pub fn justify_start(mut self) -> Self {
        let new_bundle = self.bundle.with_text_justify(JustifyText::Left);
        self.bundle = new_bundle;
        self
    }

    pub fn margin(&mut self, margin: f32) -> &mut Self {
        self.bundle.style.margin = UiRect {
            left: Val::Px(margin),
            right: Val::Px(margin),
            top: Val::Px(margin),
            bottom: Val::Px(margin),
        };
        self
    }

    pub fn set_text<S: Into<String> + Clone>(&mut self, text: S) -> &mut Text {
        self.bundle.text.sections[0].value = text.into();
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn(self.bundle);
    }

    pub fn spawn_with_payload(self, parent: &mut ChildBuilder, payload: impl Bundle) {
        parent.spawn(self.bundle).insert(payload);
    }

    const SIZE_SMALL: f32 = 18.0;
    const SIZE_MEDIUM: f32 = 36.0;
    const SIZE_LARGE: f32 = 90.0;
    const SIZE_EXTRA_LARGE: f32 = 108.0;
}


