use crate::tools::Coord;
use crate::tools::read_file;

use std::collections::HashMap;
use std::collections::HashSet;

type InputData = (Vec<Coord>, Coord);

fn parse_input(input: &str) -> InputData {
    let mut splitters = Vec::new();
    let mut start = None;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some(Coord::new(i as i32, j as i32));
            } else if c == '^' {
                splitters.push(Coord::new(i as i32, j as i32));
            }
        }
    }

    (splitters, start.unwrap())
}

fn parts(input_data: &InputData) {
    let splitters: HashSet<Coord> = input_data.0.clone().into_iter().collect();
    let mut beams: HashMap<i32, u64> = HashMap::new();
    beams
        .entry(input_data.1.j)
        .and_modify(|v| *v += 1)
        .or_insert(1);
    let mut splits = 0;

    let max_i = input_data.0.iter().map(|coord| coord.i).max().unwrap() + 1;
    for i in (input_data.1.i + 1)..=max_i {
        let mut new_beams: HashMap<i32, u64> = HashMap::new();
        for (j, count) in beams.iter() {
            if splitters.contains(&Coord::new(i, *j)) {
                new_beams
                    .entry(*j - 1)
                    .and_modify(|v| *v += count)
                    .or_insert(*count);
                new_beams
                    .entry(*j + 1)
                    .and_modify(|v| *v += count)
                    .or_insert(*count);
                splits += 1;
            } else {
                new_beams
                    .entry(*j)
                    .and_modify(|v| *v += count)
                    .or_insert(*count);
            }
        }
        beams = new_beams;
    }
    println!("day7 part1: {}", splits);
    let total = beams.values().map(|v| *v).sum::<u64>();
    println!("day7 part2: {}", total)
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day7.txt"));
    parts(&input_data);
}
