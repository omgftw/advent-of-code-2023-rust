use crate::day5;
use std::fs;

#[test]
fn test_day5_test_data() {
    let test_data = fs::read_to_string("src/day5/data/test_1.txt").unwrap();

    let result = day5::day5(Some(test_data.to_string()));
    // Part 1
    assert_eq!(result.0, 35);
    // Part 2
    // assert_eq!(result.1, 0);
}

#[test]
fn test_day5() {
    let result = day5::day5(None);
    // Part 1
    assert_eq!(result.0, 403695602);
    // Part 2
    // assert_eq!(result.1, 0);
}
