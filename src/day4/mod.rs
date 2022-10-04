use std::fs;
use std::str::Split;
use std::iter::Map;
use crate::day4::board::Board;

mod board;

pub fn run() -> Option<()> {
    let input_string = fs::read_to_string("input.txt").ok()?;

    let mut split = input_string.split("\n")
        .map(|x| String::from(x));

    let lucky_numbers = split.next()?;
    let mut boards: Vec<Board> = vec!();

    split.next()?;

    // let take = split.take(5);
    // let board = Board::from_take(take.clone());

    // loop {
    //     let mut take = split.take(5);
    //     let line_exists = take.nth(0);
    //
    //     if let None = line_exists {
    //         break;
    //     }
    //
    //     let board = Board::from_take(take.clone());
    //
    //     &split.next();
    // }



    return Some(());
}
