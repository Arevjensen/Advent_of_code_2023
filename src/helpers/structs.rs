use std::clone;

use num_traits::{AsPrimitive, PrimInt, ToPrimitive};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2D<T: PrimInt> {
    pub x: T,
    pub y: T,
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
