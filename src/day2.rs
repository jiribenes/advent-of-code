use std::cmp;

fn solve_row_easy(row: &[u32]) -> u32 {
    let first: u32 = *row.first().unwrap_or(&0);

    let (min, max) = row.iter().fold((first, first), |(min, max), &x| {
        (cmp::min(min, x), cmp::max(max, x))
    });

    max - min
}

fn solve_row_hard(row: &[u32]) -> u32 {
    for &x in row {
        for &y in row {
            if x != y && x % y == 0 {
                return x / y;
            }
        }
    }

    0
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
        .map(|row| solve_row_easy(&row))
        .sum();

    let hard: u32 = input
        .lines()
        .map(|line| parse_row(line))
        .map(|row| solve_row_hard(&row))
        .sum();

    println!("easy: {}, hard: {}", easy, hard);
}

#[test]
fn test_solve_hard() {
    assert_eq!(solve_row_hard(&vec![1, 2, 3, 4]), 2);
    assert_eq!(solve_row_hard(&vec![10, 12, 5, 7, 9]), 2);
}
