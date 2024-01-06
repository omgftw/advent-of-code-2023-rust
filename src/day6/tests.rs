use crate::day6;
use std::fs;

#[tokio::test]
async fn test_day6_test_data() {
    let test_data = fs::read_to_string("src/day6/data/test_1.txt").unwrap();

    let result = day6::day6(Some(test_data.to_string())).await;
    // Part 1
    assert_eq!(result.0, 288);
    // Part 2
    assert_eq!(result.1, 71503);
}

#[tokio::test]
async fn test_day6() {
    let result = day6::day6(None).await;
    // Part 1
    assert_eq!(result.0, 5133600);
    // Part 2
    assert_eq!(result.1, 40651271);
}
