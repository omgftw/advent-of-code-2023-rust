use crate::day9;
use std::fs;

#[tokio::test]
async fn test_day9_test_data() {
    let test_data = fs::read_to_string("src/day9/data/test_1.txt").unwrap();
    let (part1, part2) = day9::day9(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 114);
    // Part 2
    assert_eq!(part2, 2);
}

#[tokio::test]
async fn test_day9() {
    let (part1, part2) = day9::day9(None).await;

    // Part 1
    assert_eq!(part1, 1853145119);
    // Part 2
    assert_eq!(part2, 923);
}
