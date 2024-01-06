use crate::day4;
use std::fs;

#[test]
fn test_day4_test_data() {
    let test_data = fs::read_to_string("src/day4/data/test_1.txt").unwrap();

    let result = day4::day4(Some(test_data.to_string()));
    // Part 1
    assert_eq!(result.0, 13);
    // Part 2
    assert_eq!(result.1, 30);
}

#[test]
fn test_day4() {
    let result = day4::day4(None);
    // Part 1
    assert_eq!(result.0, 21558);
    // Part 2
    assert_eq!(result.1, 10425665);
}
