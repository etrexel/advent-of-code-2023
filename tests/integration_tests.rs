#[test]
fn test_day_01_part_1() {
    assert_eq!(
        "56465",
        aoc::solve(1, 1, None).expect("should return result")
    )
}

#[test]
fn test_day_01_part_2() {
    assert_eq!(
        "55902",
        aoc::solve(1, 2, None).expect("should return result")
    )
}
