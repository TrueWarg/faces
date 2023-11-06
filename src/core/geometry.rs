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
