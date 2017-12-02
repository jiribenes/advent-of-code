use std::io;
use std::io::BufRead;

pub fn solve() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap(); // this is really ugly...

    let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    {
        let shifted_one = nums.iter().cycle().skip(1);
        let result_1: u32 = nums.iter()
            .zip(shifted_one)
            .filter_map(|(x, y)| if x == y { Some(x) } else { None })
            .sum();
        println!("first: {}", result_1);
    }

    {
        let shifted_mid = nums.iter().cycle().skip(nums.len() / 2);
        let result_2: u32 = nums.iter()
            .zip(shifted_mid)
            .filter_map(|(x, y)| if x == y { Some(x) } else { None })
            .sum();
        println!("second: {}", result_2);
    }
}
