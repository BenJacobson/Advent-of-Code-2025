use crate::tools::read_file;

type Present = Vec<Vec<bool>>;

type RequiredPresents = Vec<u32>;

#[derive(Debug)]
struct Region {
    required_presents: RequiredPresents,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct InputData {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

fn parse_present(input: &str) -> Present {
    input
        .lines()
        .skip(1)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn parse_region(input: &str) -> Region {
    let mut iter = input.split(":");
    let size_raw = iter.next().unwrap();
    let size: Vec<u32> = size_raw.split("x").flat_map(&str::parse).collect();

    let required_presents = iter
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .flat_map(&str::parse)
        .collect();

    Region {
        required_presents,
        width: size[0],
        height: size[1],
    }
}

fn parse_input(input: &str) -> InputData {
    let clean_input = input.replace("\r", "");
    let mut presents_raw: Vec<&str> = clean_input.split("\n\n").collect();
    let regions_raw = presents_raw.pop().unwrap();

    let presents = presents_raw.into_iter().map(parse_present).collect();
    let regions = regions_raw.lines().map(parse_region).collect();

    InputData { presents, regions }
}

fn count_spots(region: &Region, presents: &Vec<Present>) -> u32 {
    let counts: Vec<u32> = presents
        .iter()
        .map(|present| present.iter().flatten().filter(|b| **b).count() as u32)
        .collect();

    region
        .required_presents
        .iter()
        .enumerate()
        .map(|(i, c)| (*c as u32) * counts[i])
        .sum()
}

fn part1(input_data: &InputData) {
    let lower_bound = input_data
        .regions
        .iter()
        .filter(|region| region.width * region.height >= count_spots(region, &input_data.presents))
        .count();

    let upper_bound = input_data
        .regions
        .iter()
        .filter(|region| {
            region.width * region.height >= region.required_presents.iter().sum::<u32>() * 9
        })
        .count();

    assert_eq!(lower_bound, upper_bound);

    println!("day12 part1: {}", lower_bound);
}

pub fn solve() {
    let input_data = parse_input(&read_file("inputs/day12.txt"));
    part1(&input_data);
}
