use crate::tools::read_file;

use rayon::prelude::*;
use z3::ast::Int;
use z3::{Optimize, SatResult};

struct Machine {
    indicators: u16,
    toggles: Vec<Vec<u16>>,
    joltage: Vec<u16>,
}

type InputData = Vec<Machine>;

fn parse_num_set(num_set: &str) -> Vec<u16> {
    num_set[1..num_set.len() - 1]
        .split(",")
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn parse_line(line: &str) -> Machine {
    let words: Vec<&str> = line.split_whitespace().collect();

    let indicators_str = words[0][1..words[0].len() - 1]
        .chars()
        .rev()
        .map(|c| if c == '#' { '1' } else { '0' })
        .collect::<String>();
    let indicators = u16::from_str_radix(indicators_str.as_str(), 2).unwrap();
    let toggles = words[1..words.len() - 1]
        .iter()
        .map(|num_set| parse_num_set(*num_set))
        .collect();
    let joltage = parse_num_set(words[words.len() - 1]);

    Machine {
        indicators,
        toggles,
        joltage,
    }
}

fn parse_input(input: &str) -> InputData {
    input.lines().map(parse_line).collect()
}

fn solve_machine_indicators(machine: &Machine) -> u32 {
    let mut ans: u32 = machine.toggles.len() as u32;
    for config in (0 as u32)..(1 << machine.toggles.len()) {
        let presses = config.count_ones();
        if presses >= ans {
            continue;
        }

        let mut on: u16 = 0;
        for i in 0..machine.toggles.len() {
            if config & (1 << i) != 0 {
                for j in machine.toggles[i].iter() {
                    on ^= 1 << *j;
                }
            }
        }

        if on == machine.indicators {
            ans = presses;
        }
    }
    ans
}

fn part1(input_data: &InputData) {
    let total = input_data
        .par_iter()
        .map(solve_machine_indicators)
        .sum::<u32>();
    println!("day10 part1: {}", total);
}

fn solve_machine_joltage(machine: &Machine) -> u32 {
    let toggle_vars: Vec<Int> = (0..machine.toggles.len())
        .map(|i| Int::fresh_const(i.to_string().as_str()))
        .collect();
    let solver = Optimize::new();
    for toggle_var in toggle_vars.iter() {
        solver.assert(&toggle_var.ge(0));
    }
    for ji in 0..machine.joltage.len() {
        solver.assert(
            &(0..machine.toggles.len())
                .filter(|ti| machine.toggles[*ti].contains(&(ji as u16)))
                .map(|ti| toggle_vars[ti].clone())
                .reduce(|acc, int| acc + int)
                .unwrap()
                .eq(machine.joltage[ji]),
        );
    }

    let total = Int::fresh_const("total");
    solver.assert(&total.eq(Int::add(&toggle_vars)));
    solver.minimize(&total);

    match solver.check(&[]) {
        SatResult::Sat => solver
            .get_model()
            .unwrap()
            .eval(&total, true)
            .and_then(|t| t.as_u64())
            .unwrap() as u32,
        _ => panic!("No solution"),
    }
}

fn part2(input_data: &InputData) {
    let total = input_data
        .par_iter()
        .map(solve_machine_joltage)
        .sum::<u32>();
    println!("day10 part2: {}", total);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day10.txt"));
    part1(&input_data);
    part2(&input_data);
}
