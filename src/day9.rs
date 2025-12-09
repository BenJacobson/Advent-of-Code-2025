use crate::tools::read_file;

use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;

type InputData = Vec<(i64, i64)>;

fn parse_input(input: &str) -> InputData {
    input
        .lines()
        .map(|line| {
            let r: Vec<i64> = line.split(",").map(|r| r.parse().unwrap()).collect();
            (r[0], r[1])
        })
        .collect()
}

fn part1(input_data: &InputData) {
    let total = (0..input_data.len())
        .cartesian_product(0..input_data.len())
        .map(|(i, j)| {
            ((input_data[i].0 - input_data[j].0).abs() + 1)
                * ((input_data[i].1 - input_data[j].1).abs() + 1)
        })
        .max()
        .unwrap();
    println!("day9 part1: {}", total);
}

fn compress_nums(v: &Vec<i64>) -> (HashMap<i64, usize>, HashMap<usize, i64>) {
    let compress: HashMap<i64, usize> = v
        .iter()
        .unique()
        .sorted()
        .enumerate()
        .map(|(cv, v)| (*v, cv))
        .collect();

    let decompress: HashMap<usize, i64> = v
        .iter()
        .unique()
        .sorted()
        .enumerate()
        .map(|(cv, v)| (cv, *v))
        .collect();

    (compress, decompress)
}

fn in_range(x: usize, y: usize, mut dx: i32, mut dy: i32, x_size: usize, y_size: usize) -> bool {
    dx += x as i32;
    dy += y as i32;
    0 <= dx && dx < (x_size as i32) && 0 <= dy && dy < (y_size as i32)
}

fn all_points(
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
) -> itertools::Product<std::ops::RangeInclusive<usize>, std::ops::RangeInclusive<usize>> {
    (min(x1, x2)..=max(x1, x2)).cartesian_product(min(y1, y2)..=max(y1, y2))
}

fn part2(input_data: &InputData) {
    let (compress_x, decompress_x) = compress_nums(&input_data.iter().map(|p| p.0).collect());
    let (compress_y, decompress_y) = compress_nums(&input_data.iter().map(|p| p.1).collect());
    let input_data_compressed: Vec<(usize, usize)> = input_data
        .iter()
        .map(|(x, y)| (*compress_x.get(x).unwrap(), *compress_y.get(y).unwrap()))
        .collect();

    let x_size = decompress_x.keys().max().unwrap() + 1;
    let y_size = decompress_y.keys().max().unwrap() + 1;

    let mut grid = vec![vec![0 as i8; x_size]; y_size];
    let (mut x_last, mut y_last) = *input_data_compressed.last().unwrap();
    for point in input_data_compressed.iter() {
        let (x_now, y_now) = *point;
        if x_now == x_last {
            for y in min(y_now, y_last)..=max(y_now, y_last) {
                grid[x_now][y as usize] = 1;
            }
        } else if y_now == y_last {
            for x in min(x_now, x_last)..=max(x_now, x_last) {
                grid[x as usize][y_now] = 1;
            }
        } else {
            panic!("points do not connect");
        }
        x_last = x_now;
        y_last = y_now;
    }

    for x in 0..x_size {
        for y in 0..y_size {
            if grid[x][y] != 0 {
                continue;
            }

            let mut fill: i8 = 1;
            let mut stack: Vec<(usize, usize)> = vec![(x, y)];
            let mut seen: HashSet<(usize, usize)> = HashSet::new();
            while let Some((x, y)) = stack.pop() {
                if grid[x][y] != 0 || seen.contains(&(x, y)) {
                    continue;
                }

                seen.insert((x, y));

                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    if in_range(x, y, dx, dy, x_size, y_size) {
                        stack.push(((x as i32 + dx) as usize, (y as i32 + dy) as usize));
                    } else {
                        fill = -1;
                    }
                }
            }

            for (x, y) in seen {
                grid[x][y] = fill;
            }
        }
    }

    let mut ans = 0;
    for i in 0..input_data_compressed.len() {
        for j in (i + 1)..input_data_compressed.len() {
            // this can be optimized with a "prefix" sum over the grid.
            let all_ones = all_points(
                input_data_compressed[i].0,
                input_data_compressed[j].0,
                input_data_compressed[i].1,
                input_data_compressed[j].1,
            )
            .all(|(x, y)| grid[x][y] == 1);
            if all_ones {
                let orig_x1 = decompress_x.get(&input_data_compressed[i].0).unwrap();
                let orig_y1 = decompress_y.get(&input_data_compressed[i].1).unwrap();
                let orig_x2 = decompress_x.get(&input_data_compressed[j].0).unwrap();
                let orig_y2 = decompress_y.get(&input_data_compressed[j].1).unwrap();
                let size = ((orig_x1 - orig_x2).abs() + 1) * ((orig_y1 - orig_y2).abs() + 1);
                ans = max(ans, size)
            }
        }
    }

    println!("day9 part2: {}", ans);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day9.txt"));
    part1(&input_data);
    part2(&input_data);
}
