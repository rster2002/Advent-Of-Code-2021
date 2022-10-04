use std::fs;

struct Counter {
    zeros: u32,
    ones: u32,
}

pub fn run() {
    let content = fs::read_to_string("./input/day3/test_input.txt")
        .expect("Something went wrong");
    
    let lines: Vec<&str> = content.split("\n").collect();

    let power_consumpsion = find_power_comsumpsion(lines.to_vec());
    let life_support_rating = find_life_support_rating(lines.to_vec());

    assert_eq!(power_consumpsion, 198);
    println!("Power consumpsion: {}", power_consumpsion);

    assert_eq!(life_support_rating, 230);
    println!("Life support rating: {}", life_support_rating);
}

fn find_power_comsumpsion(lines: Vec<&str>) -> u32 {
    let (gamma, epsilon) = find_gamma_and_epsilon(lines);
    return gamma * epsilon;
}

fn find_gamma_and_epsilon(lines: Vec<&str>) -> (u32, u32) {
    let mut gamma: String = String::new();
    
    // Loop over every position in the lines
    let mut i = 0;
    while i < lines[0].len() {
        // Count how many 1s and 0s there are in the current position
        let count = count_position(&lines, i);

        // Add the corrisponding character to the gamma string
        gamma.push(if count.ones >= count.zeros { '1' } else { '0' });

        i += 1;
    }

    let epsilon = invert(&gamma);

    return (
        binary_string_to_u32(gamma),
        binary_string_to_u32(epsilon) 
    );
}

fn invert(input: &String) -> String {
    let mut output = String::new();

    let mut i = 0;
    while i < input.len() {
        let character = input.chars().nth(i).expect("Invalid position");

        output.push(if character == '1' { '0' } else { '1' });

        i += 1;
    }

    return output;
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
            return binary_string_to_u32(String::from(lines[0]));
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

fn binary_string_to_u32(input: String) -> u32 {
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