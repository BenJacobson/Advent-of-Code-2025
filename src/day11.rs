use std::collections::HashMap;

use crate::tools::read_file;

type InputData = HashMap<String, Vec<String>>;

fn parse_line(line: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = line.split(":").collect();
    (
        parts[0].to_string(),
        parts[1]
            .trim()
            .split_whitespace()
            .map(&str::to_string)
            .collect(),
    )
}

fn parse_input(input: &str) -> InputData {
    input.lines().map(parse_line).collect()
}

fn count_paths_helper(curr: &str, input_data: &InputData, memo: &mut HashMap<String, u64>) -> u64 {
    if let Some(paths) = memo.get(curr) {
        return *paths;
    }

    let paths = input_data
        .get(curr)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|dest| count_paths_helper(dest, input_data, memo))
        .sum::<u64>();
    memo.insert(curr.to_string(), paths);
    paths
}

fn count_paths(start: &str, end: &str, input_data: &InputData) -> u64 {
    let mut memo: HashMap<String, u64> = HashMap::new();
    memo.insert(end.to_string(), 1);

    count_paths_helper(start, input_data, &mut memo);

    *memo.get(start).unwrap_or(&0)
}

fn part1(input_data: &InputData) {
    let ans = count_paths("you", "out", input_data);
    println!("day11 part1: {}", ans);
}

fn count_waypoint_paths(stops: &[&str], input_data: &InputData) -> u64 {
    (0..stops.len() - 1)
        .map(|i| count_paths(stops[i], stops[i + 1], input_data))
        .product::<u64>()
}

fn part2(input_data: &InputData) {
    let route1 = count_waypoint_paths(&["svr", "fft", "dac", "out"], input_data);
    let route2 = count_waypoint_paths(&["svr", "dac", "fft", "out"], input_data);
    let ans = route1 + route2;
    println!("day11 part2: {}", ans);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day11.txt"));
    part1(&input_data);
    part2(&input_data);
}
