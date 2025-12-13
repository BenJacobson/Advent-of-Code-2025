use itertools::Itertools;

use crate::tools::read_file;

use std::str::Chars;

type Equation = (Vec<Vec<char>>, fn(u64, u64) -> u64);
type InputData = Vec<Equation>;

fn parse_equation(mut equation_strs: Vec<Vec<char>>) -> Equation {
    let last = equation_strs.pop().unwrap();
    let op = match last.into_iter().find_or_first(|c| !char::is_whitespace(*c)) {
        Some('*') => |a: u64, b: u64| a * b,
        _ => |a: u64, b: u64| a + b,
    };
    (equation_strs, op)
}

fn parse_input(input: &str) -> InputData {
    let mut lines: Vec<Chars> = input.lines().map(|line| line.chars()).collect();
    let n = lines.len();

    let mut raw_equations: Vec<Vec<Vec<char>>> = Vec::new();
    raw_equations.push(vec![Vec::new(); n]);
    'outer: loop {
        let mut char_vec = Vec::new();
        for i in 0..n {
            let Some(char) = lines[i].next() else {
                break 'outer;
            };
            char_vec.push(char);
        }
        if char_vec.iter().all(|c| char::is_whitespace(*c)) {
            raw_equations.push(vec![Vec::new(); n]);
        } else {
            let last_index = raw_equations.len() - 1;
            for i in 0..n {
                raw_equations[last_index][i].push(char_vec[i]);
            }
        }
    }

    raw_equations.into_iter().map(parse_equation).collect()
}

fn compute_equation((raw_nums, op): &Equation) -> u64 {
    raw_nums
        .iter()
        .map(|v| {
            v.iter()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .reduce(op)
        .unwrap()
}

fn rotate_90(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let n = data[0].len();
    for j in 0..n {
        let mut v: Vec<char> = Vec::new();
        for i in 0..data.len() {
            v.push(data[i][j]);
        }
        result.push(v);
    }
    result
}

fn part1(input_data: &InputData) {
    let total = input_data.iter().map(compute_equation).sum::<u64>();
    println!("day6 part1: {}", total);
}

fn part2(input_data: &InputData) {
    let total = input_data
        .iter()
        .map(|(raw_nums, op)| compute_equation(&(rotate_90(raw_nums), *op)))
        .sum::<u64>();
    println!("day6 part2: {}", total);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day6.txt"));
    part1(&input_data);
    part2(&input_data);
}
