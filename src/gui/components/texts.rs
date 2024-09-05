use bevy::asset::AssetServer;
use bevy::color::palettes::css::{ANTIQUE_WHITE, DIM_GREY};
use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::prelude::{Color, Component, Entity, FlexWrap, NodeBundle, World};
use bevy::text::Text as BevyText;
use sickle_ui::prelude::{LabelConfig, UiContainerExt, UiLabelExt};
use sickle_ui::ui_builder::UiBuilder;

#[derive(Component)]
pub struct Text;

pub struct TextConfig {
    pub font_size: f32,
    pub color: Color,
}

impl Default for TextConfig {
    fn default() -> Self {
        return TextConfig {
            font_size: TEXT_SIZE_MEDIUM,
            color: Color::from(DIM_GREY),
        };
    }
}

impl TextConfig {
    pub fn from_color(color: Color) -> TextConfig {
        return TextConfig {
            font_size: TEXT_SIZE_MEDIUM,
            color,
        };
    }
}

struct SetFont(String, f32, Color);

struct SetTextSize(f32);

struct SetTextColor(Color);

trait BannerWidgetCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color,
    ) -> &mut EntityCommands<'a>;

    fn text_size(
        &'a mut self,
        size: f32,
    ) -> &mut EntityCommands<'a>;

    fn text_color(
        &'a mut self,
        color: Color,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> BannerWidgetCommands<'a> for EntityCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color,
    ) -> &mut EntityCommands<'a> {
        self.add(SetFont(font.into(), size, color))
    }

    fn text_size(&'a mut self, size: f32) -> &mut EntityCommands<'a> {
        self.add(SetTextSize(size))
    }

    fn text_color(&'a mut self, color: Color) -> &mut EntityCommands<'a> {
        self.add(SetTextColor(color))
    }
}

impl EntityCommand for SetFont {
    fn apply(self, entity: Entity, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load(&self.0);

        if let Some(mut text) = world.entity_mut(entity).get_mut::<BevyText>() {
            for text_section in &mut text.sections {
                text_section.style.font = font.clone();
                text_section.style.font_size = self.1;
                text_section.style.color = self.2;
            }
        }
    }
}

impl EntityCommand for SetTextSize {
    fn apply(self, entity: Entity, world: &mut World) {
        if let Some(mut text) = world.entity_mut(entity).get_mut::<BevyText>() {
            for text_section in &mut text.sections {
                text_section.style.font_size = self.0;
            }
        }
    }
}

impl EntityCommand for SetTextColor {
    fn apply(self, entity: Entity, world: &mut World) {
        if let Some(mut text) = world.entity_mut(entity).get_mut::<BevyText>() {
            for text_section in &mut text.sections {
                text_section.style.color = self.0;
            }
        }
    }
}

pub trait TextExt<'a> {
    fn configure_text<S: Into<String> + Clone>(&mut self, text: S, config: TextConfig) -> UiBuilder<Entity>;

    fn medium_text<S: Into<String> + Clone>(&mut self, text: S) -> UiBuilder<Entity> {
        return self.configure_text(text, TextConfig::default());
    }
}

impl<'a> TextExt<'a> for UiBuilder<'a, Entity> {
    fn configure_text<S: Into<String> + Clone>(&mut self, text: S, config: TextConfig) -> UiBuilder<Entity> {
        self.container((NodeBundle::default(), Text), |parent| {
            parent
                .label(LabelConfig {
                    label: text.into(),
                    color: ANTIQUE_WHITE.into(),
                    margin: Default::default(),
                    wrap: FlexWrap::Wrap,
                    flex_grow: 0.0,
                })
                .entity_commands()
                .font(
                    DEFAULT_FONT_PATH.to_string(),
                    config.font_size,
                    config.color,
                );
        })
    }
}

const TEXT_SIZE_SMALL: f32 = 18.0;
const TEXT_SIZE_MEDIUM: f32 = 36.0;
const TEXT_SIZE_LARGE: f32 = 90.0;
const TEXT_SIZE_EXTRA_LARGE: f32 = 108.0;

const DEFAULT_FONT_PATH: &str = "fonts/quattrocentoSans-Bold.ttf";