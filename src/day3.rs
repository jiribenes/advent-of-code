fn solve_easy(x: i32) -> i32 {
    let root = f64::from(x).sqrt().ceil() as i32;

    let layer = root / 2;
    let side_len = 2 * layer + 1;
    let smaller_sq = (2 * layer - 1).pow(2);

    let offset = (x - smaller_sq) % (side_len - 1);

    layer + (offset - layer).abs()
}

pub fn solve() {
    let target = 347_991;

    let easy = solve_easy(target);
    println!("easy: {:?}", easy);
}

#[test]
fn test_solve_easy() {
    assert_eq!(solve_easy(2), 1);
    assert_eq!(solve_easy(23), 2);
    assert_eq!(solve_easy(17), 4);
}
