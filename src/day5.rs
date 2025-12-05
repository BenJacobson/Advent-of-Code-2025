use crate::tools::read_file;

use itertools::sorted;

type InputData = (Vec<(u64, u64)>, Vec<u64>);

fn parse_input(input: &str) -> InputData {
    let sides = input.split("$").collect::<Vec<&str>>();
    let ranges = sides[0]
        .trim()
        .lines()
        .map(|line| {
            let range = line
                .split("-")
                .map(|v| v.parse().expect("Failed to parse number"))
                .collect::<Vec<u64>>();
            (range[0], range[1])
        })
        .collect();
    let items = sides[1]
        .trim()
        .lines()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();
    (ranges, items)
}

fn part1(input_data: &InputData) {
    let count = input_data
        .1
        .iter()
        .filter(|id| {
            input_data
                .0
                .iter()
                .any(|(start, end)| start <= *id && *id <= end)
        })
        .count();
    println!("day5 part1: {}", count);
}

fn part2(input_data: &InputData) {
    let mut intervals = sorted(input_data.0.iter());
    let (mut last_start, mut last_end) = intervals.next().unwrap().clone();
    let mut ans = 0;
    for (start, end) in intervals {
        assert!(start <= end);
        if *start <= last_end {
            last_end = std::cmp::max(last_end, *end);
        } else {
            ans += last_end - last_start + 1;
            last_end = *end;
            last_start = *start;
        }
    }
    ans += last_end - last_start + 1;
    println!("day5 part2: {}", ans);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day5.txt"));
    part1(&input_data);
    part2(&input_data);
}
