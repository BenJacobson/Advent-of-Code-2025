use crate::tools::read_file;

use itertools::Itertools;

type Point = (i64, i64, i64);
type Edge = (usize, usize);
type InputData = (Vec<Point>, Vec<Edge>);

fn dist(p1: &Point, p2: &Point) -> i64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let dz = p1.2 - p2.2;
    dx * dx + dy * dy + dz * dz
}

fn parse_input(input: &str) -> InputData {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let ns: Vec<i64> = line
                .split(",")
                .map(|n_str| n_str.parse().unwrap())
                .collect();
            (ns[0], ns[1], ns[2])
        })
        .collect();

    let pairs: Vec<Edge> = (0..points.len())
        .cartesian_product(0..points.len())
        .filter(|(i, j)| j > i)
        .sorted_by_cached_key(|(i, j)| dist(&points[*i], &points[*j]))
        .collect();

    (points, pairs)
}

fn part1((points, pairs): &InputData) {
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); points.len()];
    for (i, j) in pairs.iter().take(1000) {
        edges[*i].push(*j);
        edges[*j].push(*i);
    }

    let mut seen: Vec<bool> = vec![false; points.len()];
    let mut sizes: Vec<i64> = Vec::new();
    for i in 0..points.len() {
        let mut size = 0;
        let mut stack = vec![i];
        while let Some(j) = stack.pop() {
            if seen[j] {
                continue;
            }
            seen[j] = true;
            size += 1;
            for k in edges[j].iter() {
                stack.push(*k);
            }
        }

        if size > 0 {
            sizes.push(size);
        }
    }
    sizes.sort_by_key(|v| -v);

    let ans = sizes.get(0).unwrap_or(&0) * sizes.get(1).unwrap_or(&0) * sizes.get(2).unwrap_or(&0);
    println!("day8 part1: {}", ans);
}

fn part2((points, pairs): &InputData) {
    let mut group: Vec<i32> = vec![-1; points.len()];
    let mut num_groups = points.len();
    let mut next_group = 0;

    for (i, j) in pairs {
        if group[*i] == -1 && group[*j] == -1 {
            group[*i] = next_group;
            group[*j] = next_group;
            next_group += 1;
            num_groups -= 1;
        } else if group[*i] == group[*j] {
            continue;
        } else if group[*i] == -1 {
            group[*i] = group[*j];
            num_groups -= 1;
        } else if group[*j] == -1 {
            group[*j] = group[*i];
            num_groups -= 1;
        } else {
            let overwrite_group = group[*j];
            for k in 0..group.len() {
                if group[k] == overwrite_group {
                    group[k] = group[*i];
                }
            }
            num_groups -= 1;
        }

        if num_groups == 1 {
            let ans = points[*i].0 * points[*j].0;
            println!("day8 part2: {}", ans);
            break;
        }
    }
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day8.txt"));
    part1(&input_data);
    part2(&input_data);
}
