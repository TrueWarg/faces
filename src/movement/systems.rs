use super::component::{Blocks, MoveAgent, Point2D, Square, Target};
use bevy::{ecs::system::Query, utils::HashSet};
use std::collections::{HashMap, VecDeque};

pub fn route_build(
    target: Query<&Target>,
    blocks: Query<&Blocks>,
    mut agents: Query<&mut MoveAgent>,
) {
    let target = target.get_single().expect("Expected one target");
    let blocks = blocks.get_single().expect("Expected one blocks");

    for mut agent in agents.iter_mut() {
        agent.route = rebuild_route(&agent.square, &target.square, blocks);
    }
}

fn rebuild_route(start: &Square, target: &Square, blocks: &Blocks) -> Vec<Point2D> {
    let mut points_to_route: HashMap<Point2D, Vec<Point2D>> = HashMap::new();
    let mut points: VecDeque<Point2D> = VecDeque::new();
    let mut handle_points: HashSet<Point2D> = HashSet::new();

    points.push_back(start.center_position);
    handle_points.insert(start.center_position);

    let mut current_point = start.center_position;

    loop {
        if points.is_empty() {
            break;
        }

        current_point = points
            .pop_back()
            .expect("Unexpecred error: points is empty");

        if there_is_direct_route(&current_point, &target.center_position, blocks) {
            break;
        }

        handle_points.remove(&current_point);

        let near_points = extract_neighborhood(&start.center_position, blocks);
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

fn extract_neighborhood(point: &Point2D, blocks: &Blocks) -> Vec<Point2D> {
    todo!()
}

fn can_be_occupied(point: &Point2D, blocks: &Blocks) -> bool {
    return true;
}

fn there_is_direct_route(start: &Point2D, target: &Point2D, blocks: &Blocks) -> bool {
    todo!()
}

fn build_direct_route(start: &Square, target: &Square) -> Vec<Point2D> {
    todo!()
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
    let start = Square {
        half_size: 1,
        center_position: Point2D::new(3, -2),
    };

    let target = Square {
        half_size: 1,
        center_position: Point2D::new(-3, 3),
    };

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![
        Point2D::new(2, -1),
        Point2D::new(1, 0),
        Point2D::new(0, 0),
        Point2D::new(-1, 1),
        Point2D::new(-2, 2),
        Point2D::new(-3, 3),
    ];

    assert_eq!(result, expected);
}

//
//            | * |
//            | * |
//  | ^ |
//  | ^ |
//
#[test]
fn build_route_target_top_right_1() {
    let start = Square {
        half_size: 2,
        center_position: Point2D::new(0, 0),
    };

    let target = Square {
        half_size: 2,
        center_position: Point2D::new(4, 3),
    };

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![Point2D::new(2, 2), Point2D::new(4, 3)];

    assert_eq!(result, expected);
}

//
//
//
//  | ^ |      | * |
//  | ^ |
//
#[test]
fn build_route_target_horizontal_different_sizes_1() {
    let start = Square {
        half_size: 2,
        center_position: Point2D::new(1, 1),
    };

    let target = Square {
        half_size: 1,
        center_position: Point2D::new(6, 2),
    };

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![Point2D::new(3, 1), Point2D::new(5, 1)];

    assert_eq!(result, expected);
}

//      | * |
//      | * |
//
//       |^|
//
//
#[test]
fn build_route_target_vertical_different_sizes_1() {
    let start = Square {
        half_size: 1,
        center_position: Point2D::new(0, 0),
    };

    let target = Square {
        half_size: 2,
        center_position: Point2D::new(0, 5),
    };

    let blocks = Blocks::from(vec![]);

    let result = rebuild_route(&start, &target, &blocks);
    let expected = vec![
        Point2D::new(0, 1),
        Point2D::new(0, 2),
        Point2D::new(0, 3),
        Point2D::new(0, 4),
    ];

    assert_eq!(result, expected);
}
