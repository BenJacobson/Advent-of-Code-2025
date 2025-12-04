use crate::tools::read_file;

use rayon::prelude::*;
use std::iter;

type InputData = Vec<(u64, u64)>;

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

fn extract_digits(mut n: u64, digit_buffer: &mut Vec<u8>) {
    digit_buffer.clear();

    while n > 0 {
        digit_buffer.push((n % 10) as u8);
        n /= 10;
    }
}

fn has_double_sequence(n: u64, digit_buffer: &mut Vec<u8>) -> bool {
    extract_digits(n, digit_buffer);
    let half = digit_buffer.len() / 2;
    return digit_buffer[0..half] == digit_buffer[half..digit_buffer.len()];
}

fn has_repeated_sequence(n: u64, digit_buffer: &mut Vec<u8>) -> bool {
    extract_digits(n, digit_buffer);
    for i in 1..=(digit_buffer.len() / 2) {
        if digit_buffer.len() % i != 0 {
            continue;
        }
        let mut all_match = true;
        for j in 1..(digit_buffer.len() / i) {
            let start = i * j;
            if digit_buffer[0..i] != digit_buffer[start..start + i] {
                all_match = false;
                break;
            }
        }
        if all_match {
            return true;
        }
    }
    false
}

fn part1(input_data: &InputData) {
    let count: u64 = input_data
        .par_iter()
        .map(|(start, end)| {
            let mut digit_buffer = Vec::new();
            (*start..=*end)
                .filter(|n| has_double_sequence(*n, &mut digit_buffer))
                .sum::<u64>()
        })
        .sum();
    println!("day2 part1: {}", count);
}

fn part2(input_data: &InputData) {
    let count: u64 = input_data
        .par_iter()
        .map(|(start, end)| {
            let mut digit_buffer = Vec::new();
            (*start..=*end)
                .filter(|n| has_repeated_sequence(*n, &mut digit_buffer))
                .sum::<u64>()
        })
        .sum();
    println!("day2 part2: {}", count);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day2.txt"));
    part1(&input_data);
    part2(&input_data);
}
