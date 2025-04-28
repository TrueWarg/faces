use bevy::asset::Handle;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::{default, ButtonBundle, Image, ImageBundle, UiImage};
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiContainerExt;

#[derive(Component)]
pub struct EnemyId(pub usize);

#[derive(Component)]
pub struct EnemyHealth;

pub trait EnemyItemExt<'a> {
    fn enemy_item(&mut self, id: EnemyId, image: Handle<Image>) -> UiBuilder<Entity>;
}

impl<'a> EnemyItemExt<'a> for UiBuilder<'a, Entity> {
    fn enemy_item(&mut self, id: EnemyId, image: Handle<Image>) -> UiBuilder<Entity> {
        let mut item = self.container((ButtonBundle::default(), id), |parent| {
            parent.container(
                ImageBundle {
                    image: UiImage {
                        texture: image,
                        ..default()
                    },
                    ..default()
                },
                |_| {},
            );
        });
        item
    }
}
