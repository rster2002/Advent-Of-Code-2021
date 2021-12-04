use std::fs;

struct Position {
    horizonal: u32,
    depth: u32,
    aim: u32,
}

pub fn run() {
    let content = fs::read_to_string("./input/day2/actual_input.txt")
        .expect("Something went wrong");

    let lines: Vec<&str> = content.split("\n").collect();
    let position = execute_instructions(lines);

    println!("Output: {}", position.horizonal * position.depth);
}

fn execute_instructions(instructions: Vec<&str>) -> Position {
    let mut position = Position {
        horizonal: 0,
        depth: 0,
        aim: 0,
    };

    for instruction in instructions {
        let split: Vec<&str> = instruction.split(" ").collect();
        let direction = split[0];
        let amount: u32 = split[1].parse().expect("Not a number");

        match direction {
            "forward" => {
                position.horizonal += amount;
                position.depth += amount * position.aim;
            },
            "down" => position.aim += amount,
            "up" => position.aim -= amount,
            _ => println!("Ingnoring instruction"),
        }
    }

    return position;
}