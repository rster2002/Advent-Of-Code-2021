use std::fs;

struct Counter {
    zeros: u32,
    ones: u32,
}

pub fn run() {
    let content = fs::read_to_string("./input/day3/actual_input.txt")
        .expect("Something went wrong");
    
    let lines: Vec<&str> = content.split("\n").collect();
    let rating = find_life_support_rating(lines);
    // let (gamma, epsilon) = parse_input(lines);
    // let power_consumpsion = gamma * epsilon;

    println!("rating: {}", rating);

    // println!("{} * {} = {}", gamma, epsilon, power_consumpsion);
}

fn find_life_support_rating(lines: Vec<&str>) -> u32 {
    let oxygen_rating = calculate_rating(lines.to_vec(), true);
    let co2_rating = calculate_rating(lines.to_vec(), false);

    return oxygen_rating * co2_rating;
}

fn calculate_rating(l: Vec<&str>, target_ones: bool) -> u32 {
    let mut lines = l;
    let mut position = 0;

    while position < lines[0].len() {
        let count = count_position(&lines, position);
        let target_char: char;

        if count.ones >= count.zeros {
            target_char = if target_ones { '1' } else { '0' };
        } else {
            target_char = if target_ones { '0' } else { '1' };
        }

        filter_position(&mut lines, position, target_char);

        if lines.len() == 1 {
            return binary_string_to_u32(lines[0]);
        }

        position += 1;
    }

    return 0;
}

fn count_position(lines: &Vec<&str>, position: usize) -> Counter {
    let mut counter = Counter {
        zeros: 0,
        ones: 0,
    };

    for line in lines {
        let character = line.chars().nth(position).expect("Invalid position");

        match character {
            '0' => counter.zeros += 1,
            '1' => counter.ones += 1,
            _ => println!("Unmatched character"),
        }
    }

    return counter;
}

fn filter_position(lines: &mut Vec<&str>, position: usize, character: char) {
    let mut i = 0;
    loop {
        if i >= lines.len() {
            break;
        }

        let line = lines[i];
        let current_char = line.chars().nth(position).expect("Invalid position");

        if current_char != character {
            lines.remove(i);
        } else {
            i += 1;
        }
    }
}

// fn parse_input(lines: Vec<&str>) -> (u32, u32) {
//     let mut gamma: String = String::new();
//     let mut epsilon: String = String::new();
    
//     let mut i = 0;
//     while i < lines[0].len() {
//         let mut counter = Counter {
//             zeros: 0,
//             ones: 0,
//         };

//         for line in &lines {
//             let character = line.chars().nth(i).expect("Invalid position");

//             match character {
//                 '0' => counter.zeros += 1,
//                 '1' => counter.ones += 1,
//                 _ => println!("Unmatched character"),
//             }
//         }

//         if counter.ones >= counter.zeros {
//             gamma.push_str("1");
//             epsilon.push_str("0");
//         } else {
//             gamma.push_str("0");
//             epsilon.push_str("1");
//         }

//         i += 1;
//     }

//     return (
//         binary_string_to_u32(gamma),
//         binary_string_to_u32(epsilon) 
//     );
// }

// fn find_life_support_rating(lines: Vec<&str>) -> u32 {
//     let mut i = 0;

//     let mut oxygen_rating = lines.to_vec();
//     let mut co2_scrubber_rating = lines.to_vec();

//     while i < lines[0].len() {
//         let counter = count_position(&lines, i);

//         let oxygen_char: char;
//         let co2_char: char;

//         if counter.ones >= counter.zeros {
//             oxygen_char = '1';
//             co2_char = '0';
//         } else {
//             oxygen_char = '0';
//             co2_char = '1';
//         }

//         filter_position(&mut oxygen_rating, i, oxygen_char);
//         filter_position(&mut co2_scrubber_rating, i, co2_char);

//         i += 1;
//     }

//     println!("{}, {}", oxygen_rating.len(), co2_scrubber_rating.len());

//     return 0;
// }

fn binary_string_to_u32(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut i = 0;
    while i < input.len() {
        let character = input.chars().nth(i).expect("Invalid position");
        let bit_position = input.len() - i - 1;

        if character == '1' {
            let base: u32 = 2;
            result += base.pow(bit_position as u32);
        }

        i += 1;
    }

    return result;
}