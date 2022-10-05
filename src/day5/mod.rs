use std::{env, fs};
use std::cmp::min;
use crate::day5::bounds::Bounds;
use self::board::Board;
use self::point::Point;

mod board;
mod point;
mod bounds;
mod helpers;

pub fn run() -> Option<()> {
    // Read the file path for the input file
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1)?;

    // Get the contents of the file and split it into lines
    let file_contents = fs::read_to_string(input_file).ok()?;
    let mut lines = file_contents.split("\n")
        .into_iter();

    let mut segments = vec!();

    // While there is content
    while let Some(line) = lines.next() {
        // If there is no -> the line is empty and at the end of the available lines./
        if !line.contains(" -> ") {
            break;
        }

        let mut coords = line.split(" -> ");

        let start = parse_coord(coords.next()?)?;
        let end = parse_coord(coords.next()?)?;

        if start.x != end.x && start.y != end.y {
            continue;
        }

        segments.push((start, end));
    }

    let bounds = calculate_bounds(&segments);
    let mut board = Board::new(bounds.get_width(), bounds.get_height());

    for (start, end) in segments {
        board.apply_segment(&start, &end);
    }

    board.print();
    println!("Number of dangers: {}", board.nr_of_points_above_threshold(2));

    return Some(());
}

fn parse_coord(input: &str) -> Option<Point> {
    let mut parts = input.split(",");

    let x: u32 = parts.next()?
        .parse()
        .expect("Incorrect X value");

    let y: u32 = parts.next()?
        .parse()
        .expect("Incorrect Y value");

    return Some(Point { x, y });
}

fn calculate_bounds(segments: &Vec<(Point, Point)>) -> Bounds {
    let mut min_x: u32 = u32::MAX;
    let mut max_x: u32 = 0;
    let mut min_y: u32 = u32::MAX;
    let mut max_y: u32 = 0;

    // TODO cleanup this garbage code...
    for (p1, p2) in segments {
        // X
        if p1.x < min_x {
            min_x = p1.x;
        }

        if p1.x > max_x {
            max_x = p1.x;
        }

        if p2.x < min_x {
            min_x = p2.x;
        }

        if p2.x > max_x {
            max_x = p2.x;
        }

        // Y
        if p1.y < min_y {
            min_y = p1.y;
        }

        if p1.y > max_y {
            max_y = p1.y;
        }

        if p2.y < min_y {
            min_y = p2.y;
        }

        if p2.y > max_y {
            max_y = p2.y;
        }
    }

    return Bounds::from_coords(min_x, min_y, max_x, max_y);
}
