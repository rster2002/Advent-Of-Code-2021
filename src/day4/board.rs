use std::borrow::BorrowMut;
use std::iter::Iterator;
use self::square::Square;

mod square;

pub struct Board {
    values: [Square; 25],
    last_marked_value: u8,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            values: [(); 25].map(|_| Square::new()),
            last_marked_value: 0,
        };
    }

    pub fn from_take<'a>(take: impl Iterator<Item = &'a String>) -> Board {
        let mut board = Board::new();

        for (i, row) in take.enumerate() {
            let words = row.split(" ");

            let mut j = 0;
            for string_value in words {
                if string_value != "" {
                    let number_value: u8 = string_value.parse()
                        .expect("Not a valid number");

                    board.set_value(i, j, number_value);
                    j += 1;
                }
            }
        }

        return board;
    }

    fn get_square(&self, x: usize, y: usize) -> &Square {
        let index = x * 5 + y;
        return &self.values[index];
    }

    fn get_mutable_square(&mut self, x: usize, y: usize) -> &mut Square {
        let index = x * 5 + y;
        return &mut self.values[index];
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: u8) {
        self.get_mutable_square(x, y).set_value(value);
    }

    pub fn get_value(&self, x: usize, y: usize) -> u8 {
        return self.get_square(x, y).get_value();
    }

    pub fn mark_value(&mut self, value: u8) {
        self.last_marked_value = value;
        let values = &mut self.values;

        for square in values {
            if square.get_value() == value {
                square.mark();
            }
        }
    }

    pub fn is_marked(&self, x: usize, y: usize) -> bool {
        return self.get_square(x, y).is_marked();
    }

    pub fn won(&self) -> bool {
        for y in 0..5 {
            let passed = self.is_marked(0, y)
                && self.is_marked(1, y)
                && self.is_marked(2, y)
                && self.is_marked(3, y)
                && self.is_marked(4, y);

            if passed {
                return true;
            }
        }

        for x in 0..5 {
            let passed = self.is_marked(x, 0)
                && self.is_marked(x, 1)
                && self.is_marked(x, 2)
                && self.is_marked(x, 3)
                && self.is_marked(x, 4);

            if passed {
                return true;
            }
        }

        return false;
    }

    pub fn print(&self) {
        for x in 0..5 {
            for y in 0..5 {
                self.get_square(x, y).print();
            }

            println!();
        }
    }

    pub fn get_score(&self) -> u32 {
        let unmarked_total = self.values.iter()
            .filter(|x| !x.is_marked())
            .map(|x| x.get_value() as u32)
            .reduce(|a, x| a + x)
            .expect("Could not reduce iteration");

        return unmarked_total * self.last_marked_value as u32;
    }
}
