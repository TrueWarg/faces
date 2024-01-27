use crate::core::geometry::{nearest_to_point, BBox, Point2D};

use super::component::{Blocks, MoveAgent, Target};
use bevy::{ecs::system::Query, transform::components::Transform, utils::HashSet};
use std::collections::{HashMap, VecDeque};

pub fn route_build(
    target: Query<(&Target, &Transform)>,
    blocks: Query<&Blocks>,
    mut agents: Query<(&mut MoveAgent, &Transform)>,
) {
    let (target, target_transform) = target.get_single().expect("Expected one target");
    // let blocks = blocks.get_single().expect("Expected one blocks");

    for (mut agent, transform) in agents.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;
        let square = BBox::from_square(x, y, agent.half_size as f32);

        let target_x = target_transform.translation.x;
        let target_y = target_transform.translation.y;
        let target_square = BBox::from_square(target_x, target_y, target.half_size as f32);
        // todo: use rebuild_route instead of this when it complete
        agent.route = build_direct_route(&square, &target_square);
        // rebuild_route(&square, &target_square, &blocks);
    }
}

fn rebuild_route(start: &BBox, target: &BBox, blocks: &Blocks) -> Vec<Point2D> {
    let mut points_to_route: HashMap<Point2D, Vec<Point2D>> = HashMap::new();
    let mut points: VecDeque<Point2D> = VecDeque::new();
    let mut handle_points: HashSet<Point2D> = HashSet::new();

    points.push_back(start.round_center());
    handle_points.insert(start.round_center());

    let mut current_point = start.round_center();

    loop {
        if points.is_empty() {
            break;
        }

        current_point = points
            .pop_back()
            .expect("Unexpecred error: points is empty");

        if there_is_direct_route(&current_point, &target.round_center(), blocks) {
            break;
        }

        handle_points.remove(&current_point);

        let near_points = extract_available_neighborhood(&start, blocks);
        let mut curent_route = points_to_route
            .entry(current_point)
            .or_insert(vec![current_point])
            .clone();

        for point in near_points {
            let point_route = points_to_route.entry(point).or_insert(vec![]);
            if point_route.is_empty() || point_route.len() < curent_route.len() + 1 {
                curent_route.push(point);
                points_to_route.insert(point, curent_route.clone());
            }

            if !handle_points.contains(&point) {
                points.push_back(point);
                handle_points.insert(point);
            }
        }
    }

    return points_to_route.remove(&current_point).unwrap_or_default();
}

fn extract_available_neighborhood(square: &BBox, blocks: &Blocks) -> Vec<Point2D> {
    // todo: optimize do not using copy
    let mut result = Vec::with_capacity(8);
    let candidates = extract_neighborhood(square);
    for point in candidates {
        // See NOTE_SQUARE
        let rect = BBox::from_square(point.x as f32, point.y as f32, square.half_w());
        if can_be_occupied(&rect, &blocks) {
            result.push(point);
        }
    }
    return result;
}

fn extract_neighborhood(square: &BBox) -> Vec<Point2D> {
    let Point2D { x, y } = square.round_center();
    // NOTE_SQUARE:
    // bbox considered to be square
    // todo: make it more typesafe
    let size = 2 * square.half_w() as i32;
    return vec![
        Point2D::new(x - size, y + size),
        Point2D::new(x - size, y),
        Point2D::new(x - size, y - size),
        Point2D::new(x, y - size),
        Point2D::new(x + size, y - size),
        Point2D::new(x + size, y),
        Point2D::new(x + size, y + size),
        Point2D::new(x, y + size),
    ];
}

fn can_be_occupied(rect: &BBox, blocks: &Blocks) -> bool {
    let result = true;
    for block in &blocks.blocks {
        if block.round_intersection_with(rect) > 0 {
            return false;
        }
    }
    return result;
}

fn there_is_direct_route(start: &Point2D, target: &Point2D, blocks: &Blocks) -> bool {
    todo!()
}

fn build_direct_route(start: &BBox, target: &BBox) -> Vec<Point2D> {
    let mut current = *start;
    let mut route: Vec<Point2D> = vec![];

    loop {
        if current.round_intersection_with(target) > 0 {
            break;
        }

        // todo: use neihtboors restricted with blocks `extract_available_neighborhood``
        let near = extract_neighborhood(&current);
        let point = nearest_to_point(&near, &target.round_center());
        route.push(point);
        // See NOTE_SQUARE
        current = BBox::from_square(point.x as f32, point.y as f32, current.half_w());
    }

    return route;
}

// |*| - target
// |^| - current start

//
//  |*|
//
//
//           |^|
//
#[test]
fn build_route_target_top_left_1() {
    let start = BBox::from_square(3.0, -2.0, 1.0);
    let target = BBox::from_square(-3.0, 3.0, 1.0);

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![Point2D::new(1, 0), Point2D::new(-1, 2), Point2D::new(-3, 4)];

    assert_eq!(result, expected);
}

//
//            |*|
//
//  |^|
//
#[test]
fn build_route_target_top_right_1() {
    let start = BBox::from_square(0.0, 0.0, 1.0);
    let target = BBox::from_square(4.0, 3.0, 1.0);

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![Point2D::new(2, 2), Point2D::new(4, 2)];

    assert_eq!(result, expected);
}

//
//            |**|
//
//  |^|
//
#[test]
fn build_route_target_top_right_different_sizes_1() {
    let start = BBox::from_square(0.0, 0.0, 1.0);
    let target = BBox::from_square(12.0, 6.0, 2.0);

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![
        Point2D::new(2, 2),
        Point2D::new(4, 4),
        Point2D::new(6, 6),
        Point2D::new(8, 6),
        Point2D::new(10, 6),
    ];
    assert_eq!(result, expected);
}

//
//
//
//  |^|      |*|
//
//
#[test]
fn build_route_target_horizontal_1() {
    let start = BBox::from_square(1.0, 1.0, 1.0);
    let target = BBox::from_square(6.0, 2.0, 1.0);

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![Point2D::new(3, 1), Point2D::new(5, 1)];

    assert_eq!(result, expected);
}

//       |*|
//
//
//       |^|
//
//
#[test]
fn build_route_target_vertical_1() {
    let start = BBox::from_square(0.0, 0.0, 1.0);
    let target = BBox::from_square(0.0, 5.0, 1.0);

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![Point2D::new(0, 2), Point2D::new(0, 4)];

    assert_eq!(result, expected);
}

//
//           **
//           **  |*|
//           **
//  |^|      **
//
#[test]
fn build_route_with_simple_block_1() {
    let start = BBox::from_square(0.0, 0.0, 1.0);
    let target = BBox::from_square(12.0, 9.0, 1.0);

    let blocks = Blocks::from(vec![BBox::from_rect(7.0, 6.0, 2.0, 8.0)]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![
        Point2D::new(2, 0),
        Point2D::new(4, 0),
        Point2D::new(6, 0),
        Point2D::new(8, 0),
        Point2D::new(10, 2),
        Point2D::new(10, 4),
        Point2D::new(12, 6),
        Point2D::new(12, 8),
    ];

    assert_eq!(result, expected);
}

//            ** |*|
//            **
// *********  **
// *********
//  |^|   **
//        **
//        **
//
#[test]
fn build_route_with_complex_blocks_1() {
    let start = BBox::from_square(0.0, 0.0, 1.0);
    let target = BBox::from_square(12.0, 7.0, 1.0);

    let blocks = Blocks::from(vec![
        BBox::from_rect(1.0, 3.0, 8.0, 2.0),
        BBox::from_rect(0.0, 4.0, 2.0, 6.0),
        BBox::from_rect(8.0, 6.0, 2.0, 4.0),
    ]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![
        Point2D::new(2, -2),
        Point2D::new(2, -4),
        Point2D::new(4, -6),
        Point2D::new(6, -4),
        Point2D::new(8, -2),
        Point2D::new(10, 0),
        Point2D::new(12, 2),
        Point2D::new(12, 4),
        Point2D::new(12, 6),
    ];

    assert_eq!(result, expected);
}
