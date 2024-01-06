use crate::day7;
use std::fs;

#[tokio::test]
async fn test_day7_test_data() {
    let test_data = fs::read_to_string("src/day7/data/test_1.txt").unwrap();

    // Part 1
    let result = day7::day7(Some(test_data.to_string()), false).await;
    assert_eq!(result, 6592);
    // Part 2
    let result = day7::day7(Some(test_data.to_string()), true).await;
    assert_eq!(result, 6839);
}

#[tokio::test]
async fn test_day7() {
    // Part 1
    let result = day7::day7(None, false).await;
    assert_eq!(result, 241344943);
    // Part 2
    let result = day7::day7(None, true).await;
    assert_eq!(result, 243101568);
}
