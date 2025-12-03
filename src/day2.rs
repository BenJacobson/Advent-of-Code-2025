use crate::tools::read_file;

use std::iter;

type InputData = Vec<(i64, i64)>;

fn parse_input(input: &str) -> InputData {
    input
        .trim()
        .split(",")
        .map(|range| {
            let mut iter = range.split("-").map(|n| n.parse().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

fn has_double_sequence(n: &i64) -> bool {
    let s = n.to_string();
    let half = s.len() / 2;
    return s[0..half] == s[half..s.len()];
}

fn has_repeated_sequence(n: &i64) -> bool {
    let s = n.to_string();
    for i in 1..=(s.len() / 2) {
        if s.len() % i != 0 {
            continue;
        }
        let s_repeat: String = iter::repeat(s[0..i].chars())
            .flatten()
            .take(s.len())
            .collect();
        if s == s_repeat {
            return true;
        }
    }
    false
}

fn part1(input_data: &InputData) {
    let count: i64 = input_data
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(has_double_sequence)
        .sum();
    println!("day2 part1: {}", count);
}

fn part2(input_data: &InputData) {
    let count: i64 = input_data
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(has_repeated_sequence)
        .sum();
    println!("day2 part2: {}", count);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day2.txt"));
    part1(&input_data);
    part2(&input_data);
}
