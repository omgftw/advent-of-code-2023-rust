use std::fs;
use crate::day11::day11;

#[tokio::test]
async fn test_day11_test_data() {
    let test_data = fs::read_to_string("src/day11/data/test_1.txt").unwrap();
    let result = day11(Some(test_data));

    assert_eq!(result.0, 374); // Part 1
}