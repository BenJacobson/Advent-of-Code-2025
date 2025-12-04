use crate::tools::read_file;

use std::collections::HashSet;

type InputData = HashSet<(i32, i32)>;

const DIRS: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
];

fn parse_input(input: &str) -> InputData {
    let grid: Vec<&str> = input.lines().collect();
    let mut rolls = HashSet::new();

    for i in 0..grid.len() {
        let line = grid[i];
        for (j, c) in line.chars().enumerate() {
            if c != '@' {
                continue;
            }
            rolls.insert((i as i32, j as i32));
        }
    }

    rolls
}

fn part1(input_data: &InputData) {
    let count = input_data
        .iter()
        .filter(|(i, j)| {
            DIRS.iter()
                .filter(|(di, dj)| input_data.contains(&(i + di, j + dj)))
                .count()
                < 4
        })
        .count();
    println!("day4 part1: {}", count);
}

fn part2(input_data: &InputData) {
    let mut rolls: InputData = input_data.clone();
    loop {
        let mut new_rolls: InputData = rolls
            .iter()
            .filter(|(i, j)| {
                DIRS.iter()
                    .filter(|(di, dj)| rolls.contains(&(i + di, j + dj)))
                    .count()
                    >= 4
            })
            .map(|roll| *roll)
            .collect();
        if rolls.len() == new_rolls.len() {
            break;
        }
        rolls = new_rolls;
    }
    let removed = input_data.len() - rolls.len();
    println!("day4 part2: {}", removed);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day4.txt"));
    part1(&input_data);
    part2(&input_data);
}
