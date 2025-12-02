use crate::tools::read_file;

use std::iter;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    dir: Dir,
    ticks: i32,
}

const MOD: i32 = 100;

type InputData = Vec<Rotation>;

fn parse_input(input: &str) -> InputData {
    input
        .trim()
        .lines()
        .map(|line| Rotation {
            dir: match &line[0..1] {
                "L" => Dir::Left,
                _ => Dir::Right,
            },
            ticks: line[1..line.len()].parse().unwrap(),
        })
        .collect()
}

fn part1(input_data: &InputData) {
    let mut zero_count = 0;
    let mut val: i32 = 50;
    for Rotation { dir, ticks } in input_data.iter() {
        val += match dir {
            Dir::Left => -*ticks + MOD,
            Dir::Right => *ticks,
        };
        val %= MOD;

        if val == 0 {
            zero_count += 1;
        }
    }

    println!("day1 part1: {}", zero_count);
}

fn part2(input_data: &InputData) {
    let mut zero_count = 0;
    let mut val = 50;
    let sign = |rot| match rot {
        Dir::Left => -1,
        Dir::Right => 1,
    };
    let moves = input_data
        .iter()
        .map(|rot| iter::repeat(sign(rot.dir)).take(rot.ticks as usize))
        .flatten();
    for m in moves {
        val += m;
        if val % MOD == 0 {
            zero_count += 1;
        }
    }

    println!("day1 part2: {}", zero_count);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day1.txt"));
    part1(&input_data);
    part2(&input_data);
}
