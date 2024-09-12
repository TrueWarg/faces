use bevy::asset::Handle;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::prelude::{ButtonBundle, default, Image, ImageBundle, UiImage};
use bevy::prelude::Color;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiContainerExt;

#[derive(Component)]
pub struct EnemyId(pub usize);

pub trait EnemyItemExt<'a> {
    fn enemy_item(
        &mut self,
        id: EnemyId,
        image: Handle<Image>,
    ) -> UiBuilder<Entity>;
}

impl<'a> EnemyItemExt<'a> for UiBuilder<'a, Entity> {
    fn enemy_item(&mut self, id: EnemyId, image: Handle<Image>) -> UiBuilder<Entity> {
        let mut item = self.container(
            (ButtonBundle::default(), id), |parent| {
                parent
                    .container(ImageBundle {
                        image: UiImage { texture: image, ..default() },
                        ..default()
                    },
                               |_| {});
            });
        return item;
    }
}