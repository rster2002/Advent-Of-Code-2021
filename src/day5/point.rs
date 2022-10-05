use std::fmt;

pub struct Point(pub u8, pub u8);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.0, self.1);
    }
}
