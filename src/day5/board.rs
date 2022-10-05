use crate::day5::point::Point;

pub struct Board {
    positions: [u8; 100],
}

impl Board {
    pub fn new() -> Board {
        return Board {
            positions: [0; 100],
        };
    }

    // Helper functions to ensure we don't underflow
    fn abs_difference_u8(a: u8, b: u8) -> u8 {
        return if a > b {
            a - b
        } else {
            b - a
        }
    }

    pub fn apply_segment(&mut self, start: Point, end: Point) {
        let difference_x = Board::abs_difference_u8(start.0, end.0);
        let difference_y = Board::abs_difference_u8(start.1, end.1);

        if difference_x != 0 {
            for x in 0..difference_x + 1 {
                println!("{}, {}", x, start.1);
            }
        }

        // for x in 0..difference_x {
        //     self.increase_coords(x as usize, start.1 as usize);
        // }
        //
        // for y in 0..difference_x {
        //     self.increase_coords(start.0 as usize, y as usize);
        // }

        println!("{}, {}", difference_x, difference_y);
    }

    pub fn increase_coords(&mut self, x: usize, y: usize) {
        let index = y * 10 + x;
        self.positions[index] += 1;
    }

    pub fn nr_of_points_above_threshold(&self, threshold: u8) -> u8 {
        let mut points = 0;

        for position in self.positions {
            if position >= threshold {
                points += 1;
            }
        }

        return points;
    }

    pub fn print(&self) {
        for x in 0..10 {
            for y in 0..10 {
                let index = y * 10 + x;
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
