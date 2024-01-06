use crate::day8;
use std::fs;

#[tokio::test]
async fn test_day8_test_data() {
    let test_data = fs::read_to_string("src/day8/data/test_1.txt").unwrap();
    let test_data_2 = fs::read_to_string("src/day8/data/test_2.txt").unwrap();

    // Part 1
    let result = day8::day8(Some(test_data), false).await;
    assert_eq!(result, 6);
    // Part 2
    let result = day8::day8(Some(test_data_2), true).await;
    assert_eq!(result, 6);
}

#[tokio::test]
async fn test_day8() {
    // Part 1
    let result = day8::day8(None, false).await;
    assert_eq!(result, 21883);
    // Part 2
    let result = day8::day8(None, true).await;
    assert_eq!(result, 12833235391111);
}
