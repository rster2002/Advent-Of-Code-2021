use std::fmt;

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new() -> Point {
        return Point {
            x: 0,
            y: 0
        };
    }

    pub fn coords(x: u32, y: u32) -> Point {
        return Point {
            x,
            y
        };
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.x, self.y);
    }
}
