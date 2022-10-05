use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::{env, fs};
use std::str::Split;
use std::iter::Map;
use crate::day4::board::Board;

mod board;

pub fn run() -> Option<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1)?;

    let input_string = fs::read_to_string(input_file).ok()?;

    let mut split = input_string.split("\n")
        .map(|x| String::from(x));

    let lucky_number_row = split.next()?;
    let lucky_numbers = lucky_number_row.split(",")
        .map(|x| x.parse::<u8>().expect("Failed to parse"));

    let mut boards: Vec<Board> = vec!();

    // let split_ref = RefCell::new(split);

    let mut board_string_buffer: Vec<String> = vec!();
    while let Some(line) = split.next() {
        board_string_buffer.push(line);

        if board_string_buffer.len() == 6 {
            board_string_buffer.remove(0);

            let board = Board::from_take(board_string_buffer.iter());
            board_string_buffer.clear();

            boards.push(board);
        }
    }

    let mut board_won: Option<usize> = None;
    for lucky_number in lucky_numbers {
        for board in &mut boards {
            board.mark_value(lucky_number);
        }

        let mut any_won = false;
        for (i, board) in boards.iter().enumerate() {
            if board.won() {
                board_won = Some(i);
                any_won = true;
            }
        }

        if any_won {
            break;
        }
    }

    let won_index = board_won.expect("Did not win");
    let board = &boards[won_index];

    print!("Winning board score: {}", board.get_score());

    return Some(());
}
