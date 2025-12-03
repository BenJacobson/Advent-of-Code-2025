use crate::tools::read_file;

type InputData = Vec<String>;

fn parse_input(input: &str) -> InputData {
    input.trim().lines().map(|s| s.to_string()).collect()
}

fn get_joltage(s: &str, n: usize) -> u64 {
    if s.len() < n {
        return 0;
    }

    let digits = "9876543210";
    let mut start_index = 0;
    let mut joltage = Vec::new();
    for j in 0..n {
        let digit_index = start_index
            + digits
                .chars()
                .flat_map(|d| s[start_index..(s.len() - n + j + 1)].find(d))
                .next()
                .unwrap();
        joltage.push(digit_index);
        start_index = digit_index + 1;
    }

    let mut ans: u64 = 0;
    for index in joltage {
        ans *= 10;
        ans += s.chars().nth(index).unwrap().to_digit(10).unwrap() as u64;
    }
    ans
}

fn part1(input_data: &InputData) {
    let total_joltage: u64 = input_data.iter().map(|s| get_joltage(s, 2)).sum();
    println!("day3 part1: {}", total_joltage);
}

fn part2(input_data: &InputData) {
    let total_joltage: u64 = input_data.iter().map(|s| get_joltage(s, 12)).sum();
    println!("day3 part2: {}", total_joltage);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day3.txt"));
    part1(&input_data);
    part2(&input_data);
}
