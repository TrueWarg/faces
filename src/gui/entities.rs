use bevy::prelude::Component;

#[derive(Component)]
pub struct SelectorItem {
    pub name: String,
    pub description: String,
}

pub trait GetSelectorItem {
    fn selector_item(&self) -> SelectorItem {
        return SelectorItem {
            name: self.localised_name(),
            description: self.localised_description(),
        };
    }
    fn localised_name(&self) -> String;
    fn localised_description(&self) -> String;
}