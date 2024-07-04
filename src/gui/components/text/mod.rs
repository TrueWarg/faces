use bevy::prelude::*;

mod dynamic;
mod simple;

pub use dynamic::{DynamicText, DynamicTextId};
pub use simple::SimpleText;

use bevy::prelude::*;

pub trait Text: Default {
    fn for_each_section(&mut self, f: impl FnMut(&mut TextSection)) {
        self.text_bundle().text.sections.iter_mut().for_each(f);
    }

    fn small<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut text = Self::default();

        text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_SMALL;
        });

        text
    }

    fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut text = Self::default();

        text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_MEDIUM;
        });

        text
    }

    fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut text = Self::default();

        text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_LARGE;
        });

        text
    }

    fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut text = Self::default();

        text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_EXTRA_LARGE;
        });

        text
    }

    fn color(&mut self, color: Color) -> &mut Self {
        self.for_each_section(|section| section.style.color = color);
        self
    }

    fn text_bundle(&mut self) -> &mut TextBundle;

    fn spawn(self, parent: &mut ChildBuilder);

    const SIZE_SMALL: f32 = 18.0;
    const SIZE_MEDIUM: f32 = 36.0;
    const SIZE_LARGE: f32 = 90.0;
    const SIZE_EXTRA_LARGE: f32 = 108.0;
}
