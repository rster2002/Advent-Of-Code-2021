use std::fs;

pub fn run() {
    let content = fs::read_to_string("./input/day1/actual_input.txt")
        .expect("Something went wrong");

    let lines: Vec<&str> = content.split("\n").collect();
    let mut input: Vec<u32> = vec![];
    for value in lines {
        let number: u32 = value.parse().unwrap();
        input.push(number);
    }

    let sets = collect_sums(input);
    let output = count_increases(sets);

    println!("Increases: {}", output);
}

fn collect_sums(measurements: Vec<u32>) -> Vec<u32> {
    let mut result = vec![];

    let mut i = 2;
    loop {
        if i >= measurements.len() {
            break;
        }

        let a = measurements[i];
        let b = measurements[i - 1];
        let c = measurements[i - 2];

        result.push(a + b + c);
        i += 1;
    }

    return result;
}

fn count_increases(measurements: Vec<u32>) -> u32 {
    let mut result = 0;

    let mut i = 1;
    loop {
        if i >= measurements.len() {
            break;
        }

        let last = i - 1;
        let last_value = measurements[last];
        let current_value = measurements[i];

        if current_value > last_value {
            result += 1;
        }

        i += 1;
    };

    return result;
}