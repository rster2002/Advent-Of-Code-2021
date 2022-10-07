use std::ops::Sub;

// Helper functions to ensure we don't underflow
// Sub<Output = T> ensures any a type that can be subtracted can be passed
// PartialOrd ensures the type can be compared using '<', '>' etc
pub fn abs_difference<T: Sub<Output = T> + PartialOrd>(a: T, b: T) -> T {
    return if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn lowest_of<T: PartialOrd>(a: T, b: T) -> T {
    return if a < b {
        a
    } else {
        b
    }
}
