use crate::tools::read_file;

type InputData = Vec<Vec<bool>>;

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
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn get_data(grid: &InputData, i: i32, j: i32) -> bool {
    0 <= i
        && i < grid.len() as i32
        && 0 <= j
        && j < grid[i as usize].len() as i32
        && grid[i as usize][j as usize]
}

fn part1(input_data: &InputData) {
    let count = input_data
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, v)| (i, j, *v)))
        .filter(|(i, j, v)| {
            *v && DIRS
                .iter()
                .filter(|(di, dj)| get_data(input_data, (*i as i32) + di, (*j as i32) + dj))
                .count()
                < 4
        })
        .count();
    println!("day4 part1: {}", count);
}

fn part2(input_data: &InputData) {
    let mut grid: InputData = input_data.clone();
    loop {
        let mut modified = false;
        for i in 0..grid.len() {
            let line = &grid[i];
            for j in 0..line.len() {
                if !grid[i][j] {
                    continue;
                }
                let clear = DIRS
                    .iter()
                    .filter(|(di, dj)| get_data(&grid, (i as i32) + di, (j as i32) + dj))
                    .count()
                    < 4;
                if clear {
                    grid[i][j] = false;
                    modified = true;
                }
            }
        }
        if !modified {
            break;
        }
    }
    let removed = input_data.iter().flatten().filter(|v| **v).count()
        - grid.iter().flatten().filter(|v| **v).count();
    println!("day4 part2: {}", removed);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day4.txt"));
    part1(&input_data);
    part2(&input_data);
}
