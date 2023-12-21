use std::cmp::{max, min};

#[derive(Debug)]
pub struct BBox {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl BBox {
    pub fn intersection_with(&self, other: &BBox) -> i32 {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Square {
    pub half_size: i32,
    pub center_position: Point2D,
}

// Ax + Bc + C = 0
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
            b: point1.x - point1.x,
            c: point1.y * point2.x - point1.x * point2.y,
        };
    }
}

pub struct Rectangle {
    pub half_width: i32,
    pub half_height: i32,
    pub center_position: Point2D,
}

impl Rectangle {
    pub fn new(h_width: i32, h_height: i32, point: Point2D) -> Self {
        return Rectangle {
            half_width: h_width,
            half_height: h_height,
            center_position: point,
        };
    }

    pub fn sqaure(h_size: i32, point: Point2D) -> Self {
        return Rectangle {
            half_width: h_size,
            half_height: h_size,
            center_position: point,
        };
    }

    pub fn unit(point: Point2D) -> Self {
        return Rectangle {
            half_width: 1,
            half_height: 1,
            center_position: point,
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

pub fn are_intercepted(square1: &Square, sqaure2: &Square) -> bool {
    let Point2D { x, y } = square1.center_position;
    let h_size = square1.half_size;
    let bbox1 = BBox {
        left: (x - h_size) as f32,
        top: (y + h_size) as f32,
        right: (x + h_size) as f32,
        bottom: (y - h_size) as f32,
    };

    let Point2D { x, y } = sqaure2.center_position;
    let h_size = sqaure2.half_size;
    let bbox2 = BBox {
        left: (x - h_size) as f32,
        top: (y + h_size) as f32,
        right: (x + h_size) as f32,
        bottom: (y - h_size) as f32,
    };

    let kek = bbox1.intersection_with(&bbox2);

    return bbox1.intersection_with(&bbox2) > 0;
}

//   *******
//   |  1  |
//   |  ******
//   |  |  |2|
//   |  |  | |
//    **|*** |
//       ****
#[test]
fn intersection_test_1() {
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
    assert_eq!(first.intersection_with(&second), 3)
}

//   *******
//   |  2  |
//   |  ******
//   |  |  |1|
//   |  |  | |
//   ***|*** |
//      ******
#[test]
fn intersection_test_all_2() {
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
    assert_eq!(first.intersection_with(&second), 3)
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
fn intersection_test_3() {
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
    assert_eq!(first.intersection_with(&second), 2)
}

//   *************
//   |           |
//   |     1     |
//   |           |
//   |           |
//   *************
#[test]
fn intersection_test_4() {
    let first = BBox {
        left: 0.0,
        top: 4.0,
        right: 4.0,
        bottom: 0.0,
    };
    let area = (first.right as i32 - first.left as i32) * (first.top as i32 - first.bottom as i32);
    assert_eq!(first.intersection_with(&first), area);
}

//   *************
//   |  *******  |
//   |  |  2  |  |
//   |  *******  |
//   |      1    |
//   *************
#[test]
fn intersection_test_5() {
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
    assert_eq!(first.intersection_with(&second), area);
}

//   *************
//   |           |
//   |      1    |*****
//   |           |  2 |
//   |           |    |
//   ******************
#[test]
fn intersection_test_6() {
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
    assert_eq!(first.intersection_with(&second), 0);
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
fn intersection_test_7() {
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
    assert_eq!(first.intersection_with(&second), 0);
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
fn intersection_test_8() {
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
    assert_eq!(first.intersection_with(&second), 0);
}
