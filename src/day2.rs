use std::cmp;

fn solve_row(row: &[u32]) -> u32 {
    let first: u32 = *row.first().unwrap_or(&0);

    let (min, max) = row.iter().fold((first, first), |(min, max), &x| {
        (cmp::min(min, x), cmp::max(max, x))
    });

    max - min
}

fn parse_row(line: &str) -> Vec<u32> {
    line.split('\t')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect()
}

pub fn solve() {
    let input = include_str!("../inputs/day2.in");
    let easy: u32 = input
        .lines()
        .map(|line| parse_row(line))
        .map(|row| solve_row(&row))
        .sum();

    println!("{:?}", easy);
}
