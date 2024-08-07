use bevy::app::{App, Plugin};
use bevy::ecs::query::QueryEntityError;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::Added;
use bevy::prelude::AlignItems;
use bevy::prelude::AlignSelf;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::default;
use bevy::prelude::Entity;
use bevy::prelude::EventReader;
use bevy::prelude::FlexDirection;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::Node;
use bevy::prelude::Overflow;
use bevy::prelude::Query;
use bevy::prelude::Reflect;
use bevy::prelude::Res;
use bevy::prelude::Style;
use bevy::prelude::Time;
use bevy::prelude::Touches;
use bevy::prelude::Update;
use bevy::prelude::Val;
use bevy::prelude::With;

pub struct VerticalScrollPlugin;

impl Plugin for VerticalScrollPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Scroll>()
            .register_type::<ScrollableContent>()
            .add_systems(
                Update,
                (
                    scroll_view_spawns,
                    mouse_pressed_move_input,
                    touch_pressed_move_input,
                    mouse_wheel_input,
                    content_updates,
                )
                    .chain(),
            );
    }
}


#[derive(Component, Reflect)]
pub struct Scroll {
    pub speed: f32,
}

/// NOTE_INTERACTABLE_CHILDREN_HACK
/// Now children intercept interaction and brake scroll.
/// Use it for hack to detect that any of children has interaction and allow scrolling.
/// TODO: Made some relation between [Scroll] and [InScroll] using id.
#[derive(Component)]
pub struct InScroll;

impl Default for Scroll {
    fn default() -> Self {
        Self {
            speed: 400.0,
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct ScrollableContent {
    pub y: f32,
}

fn scroll_view_spawns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Style), Added<Scroll>>,
) {
    for (entity, mut style) in query.iter_mut() {
        style.overflow = Overflow::clip();
        style.align_items = AlignItems::Start;
        style.align_self = AlignSelf::Stretch;
        style.flex_direction = FlexDirection::Row;
        commands.entity(entity).insert(Interaction::None);
    }
}

fn mouse_pressed_move_input(
    mut motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&Children, &Interaction, &Node), With<Scroll>>,
    mut content_query: Query<(&mut ScrollableContent, &Node)>,
) {
    for motion in motion_event_reader.read() {
        update_positions(&mut query, &mut content_query, motion.delta.y);
    }
}

fn touch_pressed_move_input(
    touches: Res<Touches>,
    mut query: Query<(&Children, &Interaction, &Node), With<Scroll>>,
    mut content_query: Query<(&mut ScrollableContent, &Node)>,
) {
    for touch in touches.iter() {
        let Some(touch) = touches.get_pressed(touch.id()) else {
            continue;
        };
        update_positions(&mut query, &mut content_query, touch.delta().y);
    }
}

fn update_positions(
    query: &mut Query<(&Children, &Interaction, &Node), With<Scroll>>,
    content_query: &mut Query<(&mut ScrollableContent, &Node)>,
    delta_y: f32,
) {
    for (children, &interaction, container) in query.iter_mut() {
        if interaction != Interaction::Pressed {
            continue;
        }
        let container_height = container.size().y;
        for &child in children.iter() {
            if let Ok((mut content, node)) = content_query.get_mut(child) {
                let max_scroll = (node.size().y - container_height).max(0.0);
                content.y += delta_y;
                content.y = content.y.clamp(-max_scroll, 0.0);
            }
        }
    }
}

fn mouse_wheel_input(
    mut motion_event_reader: EventReader<MouseWheel>,
    mut query: Query<(&Children, &Interaction, &Scroll, &Node), With<Scroll>>,
    // See NOTE_INTERACTABLE_CHILDREN_HACK
    interactions_query: Query<(&Interaction), With<InScroll>>,
    mut content_query: Query<(&mut ScrollableContent, &Node)>,
    time: Res<Time>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for motion in motion_event_reader.read() {
        for (children, &interaction, scroll_view, container) in query.iter_mut() {
            let y = match motion.unit {
                MouseScrollUnit::Line => {
                    motion.y * time.delta().as_secs_f32() * scroll_view.speed
                }
                MouseScrollUnit::Pixel => motion.y,
            };

            let is_any_children_hovered = interactions_query.iter().find(|interaction| {
                **interaction == Interaction::Hovered
            });

            if interaction != Interaction::Hovered && is_any_children_hovered.is_none() {
                continue;
            }
            let container_height = container.size().y;
            for &child in children.iter() {
                if let Ok((mut content, node)) = content_query.get_mut(child) {
                    let y = y * time.delta().as_secs_f32() * scroll_view.speed;
                    let max_scroll = (node.size().y - container_height).max(0.0);
                    content.y += y;
                    content.y = content.y.clamp(-max_scroll, 0.);
                }
            }
        }
    }
}

fn content_updates(
    mut query: Query<(&ScrollableContent, &mut Style), Changed<ScrollableContent>>
) {
    for (content, mut style) in query.iter_mut() {
        style.top = Val::Px(content.y);
    }
}