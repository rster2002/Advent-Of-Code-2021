use crate::day5::helpers::{abs_difference};
use crate::day5::point::Point;

pub struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

impl Bounds {
    pub fn new() -> Bounds {
        return Bounds {
            top_left: Point::coords(0, 0),
            bottom_right: Point::coords(0, 0),
        };
    }

    pub fn from_points(top_left: Point, bottom_right: Point) -> Bounds {
        return Bounds {
            top_left,
            bottom_right,
        };
    }

    pub fn from_coords(x1: u32, y1: u32, x2: u32, y2: u32) -> Bounds {
        return Bounds {
            top_left: Point::coords(x1, y1),
            bottom_right: Point::coords(x2, y2),
        };
    }

    pub fn get_width(&self) -> u32 {
        return self.bottom_right.x + 1;
    }

    pub fn get_height(&self) -> u32 {
        return self.bottom_right.y + 1;
    }

    pub fn get_top_left(&self) -> &Point {
        return &self.top_left;
    }

    pub fn get_bottom_right(&self) -> &Point {
        return &self.bottom_right;
    }
}
