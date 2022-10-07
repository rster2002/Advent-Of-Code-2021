use crate::day5::helpers::{abs_difference, lowest_of};
use crate::day5::point::Point;

pub struct Board {
    width: u32,
    height: u32,
    positions: Vec<u32>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let mut instance = Board {
            width,
            height,
            positions: vec!(),
        };

        for _ in 0..(width * height) {
            instance.positions.push(0);
        }

        return instance;
    }

    pub fn apply_segment(&mut self, start: &Point, end: &Point) {
        let difference_x = abs_difference(start.x, end.x);
        let difference_y = abs_difference(start.y, end.y);

        let origin = Point {
            x: lowest_of(start.x, end.x),
            y: lowest_of(start.y, end.y)
        };

        if difference_x != 0 {
            for x in 0..difference_x + 1 {
                self.increase_coords(
                    (origin.x + x) as usize,
                    origin.y as usize
                );
            }
        }

        if difference_y != 0 {
            for y in 0..difference_y + 1 {
                self.increase_coords(
                    origin.x as usize,
                    (origin.y + y) as usize
                );
            }
        }
    }

    pub fn increase_coords(&mut self, x: usize, y: usize) {
        let index = y * (self.width as usize) + x;
        self.positions[index] += 1;
    }

    pub fn nr_of_points_above_threshold(&self, threshold: u32) -> u32 {
        let mut points = 0;

        for position in &self.positions {
            if position >= &threshold {
                points += 1;
            }
        }

        return points;
    }

    pub fn print(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let width = self.width as usize;
                let index = y as usize * width + x as usize;
                let value = self.positions[index];

                if value == 0 {
                    print!("{}", ".");
                } else {
                    print!("{}", value);
                }
            }

            println!();
        }
    }
}
