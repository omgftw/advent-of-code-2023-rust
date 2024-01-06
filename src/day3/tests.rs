use crate::day3;
use std::fs;

#[test]
fn test_day3_test_data() {
    let test_data = fs::read_to_string("src/day3/data/test_1.txt").unwrap();

    let result = day3::day3(Some(test_data));
    // Part 1
    assert_eq!(result.0, 4361);
    // Part 2
    assert_eq!(result.1, 467835);
}

#[test]
fn test_day3() {
    let result = day3::day3(None);
    // Part 1
    assert_eq!(result.0, 525911);
    // Part 2
    assert_eq!(result.1, 75805607);
}
