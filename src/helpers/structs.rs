use std::clone;

use num_traits::{AsPrimitive, PrimInt, ToPrimitive};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl Point2D<i32> {
    pub fn up(&self) -> Point2D<i32> {
        Point2D {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Point2D<i32> {
        Point2D {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Point2D<i32> {
        Point2D {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn rigth(&self) -> Point2D<i32> {
        Point2D {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Point2D<usize> {
    pub fn up(&self) -> Option<Point2D<usize>> {
        if self.y > 0 {
            Some(Point2D {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }
    pub fn left(&self) -> Option<Point2D<usize>> {
        if self.x > 0 {
            Some(Point2D {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }
    pub fn right(&self) -> Option<Point2D<usize>> {
        Some(Point2D {
            x: self.x + 1,
            y: self.y,
        })
    }
    pub fn down(&self) -> Option<Point2D<usize>> {
        Some(Point2D {
            x: self.x,
            y: self.y + 1,
        })
    }

    pub fn right_bounded(&self, x_bound: usize) -> Option<Point2D<usize>> {
        if self.x + 1 >= x_bound {
            return None;
        }
        Some(Point2D {
            x: self.x + 1,
            y: self.y,
        })
    }
    pub fn down_bounded(&self, y_bound: usize) -> Option<Point2D<usize>> {
        if self.y + 1 >= y_bound {
            return None;
        }
        Some(Point2D {
            x: self.x,
            y: self.y + 1,
        })
    }
}

impl<T: ToPrimitive + PrimInt> Point2D<T> {
    pub fn to_grid_index(&self, upper_x: T, upper_y: T) -> Option<usize> {
        let (self_x_option, self_y_option) = (self.x.to_usize(), self.y.to_usize());
        let (upper_x_option, upper_y_option) = (upper_x.to_usize(), upper_y.to_usize());

        match (self_x_option, self_y_option, upper_x_option, upper_y_option) {
            (Some(x), Some(y), Some(upper_x), Some(upper_y)) => {
                if x < upper_x && y < upper_y {
                    Some(x + (y * upper_x))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

mod test {
    use crate::helpers::structs::Point2D;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 11)]
    #[case(5, 2, 25)]
    fn idx_inside_bounds_returns_some(#[case] x: i32, #[case] y: i32, #[case] result: usize) {
        let test_point = Point2D { x: x, y: y };
        let (upper_x, upper_y) = (10, 10);

        let function_idx = test_point.to_grid_index(upper_x, upper_y);

        assert_eq!(Some(result), function_idx);
    }
    #[rstest]
    #[case(11, 1, None)]
    #[case(5, 20, None)]
    fn idx_outside_bounds_return_none(
        #[case] x: i32,
        #[case] y: i32,
        #[case] result: Option<usize>,
    ) {
        let test_point = Point2D { x: x, y: y };
        let (upper_x, upper_y) = (10, 10);

        let function_idx = test_point.to_grid_index(upper_x, upper_y);

        assert_eq!(result, function_idx);
    }
}
