use std::cmp::{max, min};

#[derive(Clone, Copy, Debug)]
pub struct BBox {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl BBox {
    pub fn round_intersection_with(&self, other: &BBox) -> i32 {
        let x_overlap = max(
            0,
            min(self.right as i32, other.right as i32) - max(self.left as i32, other.left as i32),
        );
        let y_overlap = max(
            0,
            min(self.top as i32, other.top as i32) - max(self.bottom as i32, other.bottom as i32),
        );
        return x_overlap * y_overlap;
    }

    pub fn half_w(&self) -> f32 {
        return (self.right - self.left) / 2.0;
    }

    pub fn half_h(&self) -> f32 {
        return (self.top - self.bottom) / 2.0;
    }

    pub fn from_square(center_x: f32, center_y: f32, half_size: f32) -> Self {
        return BBox {
            left: center_x - half_size,
            top: center_y + half_size,
            right: center_x + half_size,
            bottom: center_y - half_size,
        };
    }

    pub fn from_rect(center_x: f32, center_y: f32, half_w: f32, half_h: f32) -> Self {
        return BBox {
            left: center_x - half_w,
            top: center_y + half_h,
            right: center_x + half_w,
            bottom: center_y - half_h,
        };
    }

    pub fn round_center(&self) -> Point2D {
        let half_w = self.half_w();
        let half_h = self.half_h();
        return Point2D {
            x: (self.left + half_w) as i32,
            y: (self.bottom + half_h) as i32,
        };
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Point2D {
        return Point2D { x, y };
    }

    // d = sqrt((xb - xa)^2 + (yb - ya)^2)
    pub fn distance_to(&self, other: &Point2D) -> f32 {
        let sq_x = (other.x - self.x) * (other.x - self.x);
        let sq_y = (other.y - self.y) * (other.y - self.y);
        return ((sq_x + sq_y) as f32).sqrt();
    }
}

// Ax + Bc + C = 0
#[derive(Clone, Copy, Debug)]
pub struct Line2D {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

impl Line2D {
    pub fn new(a: i32, b: i32, c: i32) -> Self {
        return Line2D { a, b, c };
    }

    // x -  xa   y -  ya
    // ------- = ------- -> Ax + By + C = 0
    // xb - xa   yb - ya
    pub fn from(point1: &Point2D, point2: &Point2D) -> Self {
        return Line2D {
            a: point2.y - point1.y,
            b: point1.x - point2.x,
            c: point1.y * point2.x - point1.x * point2.y,
        };
    }
}

pub fn nearest_to_line(points: &Vec<Point2D>, line: &Line2D) -> Point2D {
    let mut nearest = points.first().expect("points must be not empty");
    let mut distance = f32::MAX;
    for point in points {
        let new_distance = distance_to_line(point, line);
        if new_distance < distance {
            nearest = point;
            distance = new_distance;
        }
    }

    return nearest.clone();
}

pub fn nearest_to_point(points: &Vec<Point2D>, target: &Point2D) -> Point2D {
    let mut nearest = points.first().expect("points must be not empty");
    let mut distance = f32::MAX;
    for point in points {
        let new_distance = point.distance_to(target);
        if new_distance < distance {
            nearest = point;
            distance = new_distance;
        }
    }

    return nearest.clone();
}

//      |Ax0 + By0 + C|
//  d = ---------------
//      sqrt(A^2 + B^2)
pub fn distance_to_line(point: &Point2D, line: &Line2D) -> f32 {
    let numerator = (line.a * point.x + line.b * point.y + line.c).abs() as f32;
    let denominator = ((line.a * line.a + line.b * line.b) as f32).sqrt();
    return numerator / denominator;
}

/// find lines intersection with solving equations systems using Cramer's rule
/// A1x + B1y = -C1
/// A2x + B2y = -C2
///     |A1 B1|
/// d = |     | = A1 * B2 - A2 * B1;
///     |A2 B2|
///     |-C1 B1|
/// dx =|      | = -C1 * B2 + C2 * B1
///     |-C2 B2|
///     |A1 -C1|
/// dy =|      | = -A1 * C2 + A2 * C1;
///     |A2 -C2|
///     dx
/// x = --
///     d
///     dy
/// y = --
///     d
pub fn round_lines_intersection(line1: &Line2D, line2: &Line2D) -> Option<Point2D> {
    let d = line1.a * line2.b - line1.b * line2.a;
    if d == 0 {
        return None;
    }
    let dx = -line1.c * line2.b + line2.c * line1.b;
    let dy = -line1.a * line2.c + line2.a * line1.c;
    return Some(Point2D {
        x: dx / d,
        y: dy / d,
    });
}

pub fn round_segments_intersection(
    segment1_x: &Point2D,
    segment1_y: &Point2D,
    segment2_x: &Point2D,
    segment2_y: &Point2D,
) -> Option<Point2D> {
    let line1 = Line2D::from(segment1_x, segment1_y);
    let line2 = Line2D::from(segment2_x, segment2_y);

    return round_lines_intersection(&line1, &line2).and_then(|point| {
        let intersect_first = point_in_rectangle(&point, segment1_x, segment1_y);
        let intersect_second = point_in_rectangle(&point, segment2_x, segment2_y);
        if intersect_first && intersect_second {
            Some(point)
        } else {
            None
        }
    });
}

fn point_in_rectangle(point: &Point2D, top_left: &Point2D, bottom_right: &Point2D) -> bool {
    let min_x = min(top_left.x, bottom_right.x);
    let max_x = max(top_left.x, bottom_right.x);
    let min_y = min(top_left.y, bottom_right.y);
    let max_y = max(top_left.y, bottom_right.y);

    return min_x <= point.x && point.x <= max_x && min_y <= point.y && point.y <= max_y;
}

//   *******
//   |  1  |
//   |  ******
//   |  |  |2|
//   |  |  | |
//    **|*** |
//       ****
#[test]
fn bbox_round_intersection_test_1() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 2.0,
        bottom: 0.0,
    };

    let second = BBox {
        left: 1.0,
        top: 3.0,
        right: 2.5,
        bottom: -1.0,
    };
    assert_eq!(first.round_intersection_with(&second), 3)
}

//   *******
//   |  2  |
//   |  ******
//   |  |  |1|
//   |  |  | |
//   ***|*** |
//      ******
#[test]
fn bbox_round_intersection_test_2() {
    let second = BBox {
        left: 0.0,
        top: 4.0,
        right: 2.0,
        bottom: 0.0,
    };

    let first = BBox {
        left: 1.0,
        top: 3.0,
        right: 2.5,
        bottom: -1.0,
    };
    assert_eq!(first.round_intersection_with(&second), 3)
}

//      *******
//      |  2  |
//   ******** |
//   |  | ? | |
//   |  *******
//   |   1  |
//   |      |
//   ********
#[test]
fn bbox_round_intersection_test_3() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 2.0,
        bottom: 0.0,
    };

    let second = BBox {
        left: 1.0,
        top: 6.0,
        right: 2.5,
        bottom: 2.0,
    };
    assert_eq!(first.round_intersection_with(&second), 2)
}

//   *************
//   |           |
//   |     1     |
//   |           |
//   |           |
//   *************
#[test]
fn bbox_round_intersection_test_4() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 4.0,
        bottom: 0.0,
    };
    let area = (first.right as i32 - first.left as i32) * (first.top as i32 - first.bottom as i32);
    assert_eq!(first.round_intersection_with(&first), area);
}

//   *************
//   |  *******  |
//   |  |  2  |  |
//   |  *******  |
//   |      1    |
//   *************
#[test]
fn bbox_round_intersection_test_5() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 4.0,
        bottom: 0.0,
    };

    let second = BBox {
        left: 1.0,
        top: 3.0,
        right: 3.0,
        bottom: 2.0,
    };
    let area =
        (second.right as i32 - second.left as i32) * (second.top as i32 - second.bottom as i32);
    assert_eq!(first.round_intersection_with(&second), area);
}

//   *************
//   |           |
//   |      1    |*****
//   |           |  2 |
//   |           |    |
//   ******************
#[test]
fn bbox_round_intersection_test_6() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 4.0,
        bottom: 0.0,
    };

    let second = BBox {
        left: 4.0,
        top: 3.0,
        right: 6.0,
        bottom: 0.0,
    };
    assert_eq!(first.round_intersection_with(&second), 0);
}

//   ********
//   |      |
//   |   1  |
//   |      |  *******
//   |      |  |     |
//   ********  |  2  |
//             |     |
//             *******
#[test]
fn bbox_round_intersection_test_7() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 3.0,
        bottom: 0.0,
    };

    let second = BBox {
        left: 4.0,
        top: 2.0,
        right: 6.0,
        bottom: -2.0,
    };
    assert_eq!(first.round_intersection_with(&second), 0);
}

//   ********
//   |      |
//   |   2  |
//   |      |  *******
//   |      |  |     |
//   ********  |  1  |
//             |     |
//             *******
#[test]
fn bbox_round_intersection_test_8() {
    let second = BBox {
        left: 0.0,
        top: 4.0,
        right: 3.0,
        bottom: 0.0,
    };

    let first = BBox {
        left: 4.0,
        top: 2.0,
        right: 6.0,
        bottom: -2.0,
    };
    assert_eq!(first.round_intersection_with(&second), 0);
}

#[test]
fn distance_to_line_test_1() {
    let point = Point2D { x: 0, y: 0 };
    let line = Line2D::from(&Point2D { x: 2, y: -2 }, &Point2D { x: 2, y: 2 });
    assert_eq!(distance_to_line(&point, &line), 2.0);
}

#[test]
fn distance_to_line_test_2() {
    let point = Point2D { x: 0, y: 1 };
    let line = Line2D::from(&Point2D { x: -2, y: -2 }, &Point2D { x: 2, y: -2 });
    assert_eq!(distance_to_line(&point, &line), 3.0);
}

#[test]
fn distance_to_line_test_3() {
    let point = Point2D { x: 0, y: 0 };
    let line = Line2D::from(&Point2D { x: 0, y: 6 }, &Point2D { x: 4, y: 0 });
    let expected = "3.3282";
    let result = format!("{:.4}", distance_to_line(&point, &line));
    assert_eq!(result, expected);
}

#[test]
fn distance_to_line_test_4() {
    let point = Point2D { x: -2, y: 2 };
    let line = Line2D::from(&Point2D { x: -3, y: 1 }, &Point2D { x: -2, y: 0 });
    let expected = format!("{:.4}", 2.0f32.sqrt());
    let result = format!("{:.4}", distance_to_line(&point, &line));
    assert_eq!(result, expected);
}

#[test]
fn round_lines_intersection_test_1() {
    let line1 = Line2D::new(1, 1, 0);
    let line2 = Line2D::new(2, 1, 0);
    let result = round_lines_intersection(&line1, &line2);
    assert_eq!(result, Some(Point2D::new(0, 0)));
}

#[test]
fn round_lines_intersection_test_2() {
    let line1 = Line2D::new(3, 1, 2);
    let line2 = Line2D::new(2, 7, -6);
    let result = round_lines_intersection(&line1, &line2);
    assert_eq!(result, Some(Point2D::new(-1, 1)));
}

#[test]
fn round_lines_intersection_test_3() {
    let line1 = Line2D::new(0, 1, -12);
    let line2 = Line2D::new(2, 1, 6);
    let result = round_lines_intersection(&line1, &line2);
    assert_eq!(result, Some(Point2D::new(-9, 12)));
}

#[test]
fn round_lines_intersection_test_4() {
    let line1 = Line2D::new(2, 3, 0);
    let line2 = Line2D::new(-3, 1, 1);
    let result = round_lines_intersection(&line1, &line2);
    assert_eq!(result, Some(Point2D::new(0, 0)));
}

#[test]
fn round_lines_intersection_test_parallel_5() {
    let line1 = Line2D::new(1, -1, 0);
    let line2 = Line2D::new(1, -1, -2);
    let result = round_lines_intersection(&line1, &line2);
    assert_eq!(result, None);
}

#[test]
fn round_lines_intersection_test_the_same_6() {
    let line1 = Line2D::new(1, -1, -2);
    let line2 = Line2D::new(1, -1, -2);
    let result = round_lines_intersection(&line1, &line2);
    assert_eq!(result, None);
}

#[test]
fn round_segments_intersection_parallel_1() {
    let start1 = Point2D::new(0, 0);
    let end1 = Point2D::new(0, 1);

    let start2 = Point2D::new(1, 0);
    let end2 = Point2D::new(1, 1);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, None);
}

#[test]
fn round_segments_intersection_not_intersect_2() {
    let start1 = Point2D::new(0, 0);
    let end1 = Point2D::new(-1, 3);

    let start2 = Point2D::new(2, 0);
    let end2 = Point2D::new(3, 3);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, None);
}

#[test]
fn round_segments_intersection_not_intersect_3() {
    let start1 = Point2D::new(2, 1);
    let end1 = Point2D::new(0, 0);

    let start2 = Point2D::new(0, 3);
    let end2 = Point2D::new(7, 0);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, None);
}

#[test]
fn round_segments_intersection_4() {
    let start1 = Point2D::new(6, 3);
    let end1 = Point2D::new(0, 0);

    let start2 = Point2D::new(0, 3);
    let end2 = Point2D::new(7, 0);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, Some(Point2D::new(3, 1)));
}

#[test]
fn round_segments_intersection_5() {
    let start1 = Point2D::new(6, 4);
    let end1 = Point2D::new(0, 0);

    let start2 = Point2D::new(0, 4);
    let end2 = Point2D::new(6, 0);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, Some(Point2D::new(3, 2)));
}

#[test]
fn round_segments_intersection_6() {
    let start1 = Point2D::new(0, 0);
    let end1 = Point2D::new(4, 4);

    let start2 = Point2D::new(2, 5);
    let end2 = Point2D::new(2, -3);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, Some(Point2D::new(2, 2)));
}

#[test]
fn round_segments_intersection_7() {
    let start1 = Point2D::new(-3, 0);
    let end1 = Point2D::new(5, 0);

    let start2 = Point2D::new(0, 3);
    let end2 = Point2D::new(0, -4);

    let result = round_segments_intersection(&start1, &end1, &start2, &end2);
    assert_eq!(result, Some(Point2D::new(0, 0)));
}
