use std::borrow::BorrowMut;
use std::iter::Iterator;
use self::square::Square;

mod square;

pub struct Board {
    values: [Square; 25],
}

impl Board {
    pub fn new() -> Board {
        return Board {
            values: [(); 25].map(|_| Square::new()),
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
        let values = &mut self.values;

        for square in values {
            if square.get_value() == value {
                square.mark();
            }
        }
    }
}
