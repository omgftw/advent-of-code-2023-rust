use crate::day11::day11;
use std::fs;

#[tokio::test]
async fn test_day11_test_data() {
    let test_data = fs::read_to_string("src/day11/data/test_1.txt").unwrap();
    let result1 = day11(Some(test_data.clone()), 1);
    let result2 = day11(Some(test_data.clone()), 9);
    let result3 = day11(Some(test_data.clone()), 99);

    assert_eq!(result1, 374);
    assert_eq!(result2, 1030);
    assert_eq!(result3, 8410);
}

#[tokio::test]
async fn test_day11() {
    let part1 = day11(None, 1);
    let part2 = day11(None, 999_999);

    // Part 1
    assert_eq!(part1, 9522407);
    // Part 2
    assert_eq!(part2, 544723432977);
}
