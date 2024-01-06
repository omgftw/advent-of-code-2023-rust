use crate::day10;
use std::fs;

#[tokio::test]
async fn test_day10_test_data() {
    let test_data = fs::read_to_string("src/day10/data/test_1.txt").unwrap();
    let result = day10::day10(Some(test_data)).await;
    assert_eq!(result.0, 8); // Part 1
    assert_eq!(result.1, 1); // Part 2

    let test_data = fs::read_to_string("src/day10/data/test_2.txt").unwrap();
    let result = day10::day10(Some(test_data)).await;
    assert_eq!(result.0, 23); // Part 1
    assert_eq!(result.1, 4); // Part 2

    let test_data = fs::read_to_string("src/day10/data/test_3.txt").unwrap();
    let result = day10::day10(Some(test_data)).await;
    assert_eq!(result.0, 80); // Part 1
    assert_eq!(result.1, 10); // Part 2
}

#[tokio::test]
async fn test_day10() {
    let result = day10::day10(None).await;

    // Part 1
    assert_eq!(result.0, 6599);
    // Part 2
    assert_eq!(result.1, 477);
}
