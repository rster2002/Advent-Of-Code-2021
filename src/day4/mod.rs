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

    // let mut board_won: Option<usize> = None;
    let mut winning_boards: Vec<usize> = vec!();
    for lucky_number in lucky_numbers {
        for board in &mut boards {
            board.mark_value(lucky_number);
        }

        let mut any_won = false;
        for (i, board) in boards.iter().enumerate() {
            if board.won() && !winning_boards.contains(&i) {
                winning_boards.push(i);
            }
        }

        if winning_boards.len() == boards.len() {
            break;
        }
    }

    let last_index = winning_boards[winning_boards.len() - 1];
    let last_winning_board = &boards[last_index];

    last_winning_board.print();

    println!("Last winning board score: {}", last_winning_board.get_score());

    return Some(());
}
