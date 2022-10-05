use std::{env, fs};
use self::board::Board;
use self::point::Point;

mod board;
mod point;

pub fn run() -> Option<()> {
    // Read the file path for the input file
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1)?;

    // Get the contents of the file and split it into lines
    let file_contents = fs::read_to_string(input_file).ok()?;
    let mut lines = file_contents.split("\n")
        .into_iter();

    // Create a board
    let mut board = Board::new();

    // While there is content
    while let Some(line) = lines.next() {
        // If there is no -> the line is empty and at the end of the available lines./
        if !line.contains(" -> ") {
            break;
        }

        let mut coords = line.split(" -> ");

        let start = parse_coord(coords.next()?)?;
        let end = parse_coord(coords.next()?)?;

        board.apply_segment(start, end);
    }

    board.print();
    println!("Number of dangers: {}", board.nr_of_points_above_threshold(2));

    return Some(());
}

fn parse_coord(input: &str) -> Option<Point> {
    let mut parts = input.split(",");

    let x: u8 = parts.next()?
        .parse()
        .expect("Incorrect X value");

    let y: u8 = parts.next()?
        .parse()
        .expect("Incorrect Y value");

    return Some(Point(x, y));
}
