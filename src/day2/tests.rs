use crate::day2;
use std::fs;

#[test]
fn test_day2_test_data() {
    let test_data = fs::read_to_string("src/day2/data/test_1.txt").unwrap();

    // Part 1
    assert_eq!(day2::day2(12, 13, 14, Some(test_data.clone())).0, 8);
    // Part 2
    assert_eq!(day2::day2(12, 13, 14, Some(test_data)).1, 2286);
}

#[test]
fn test_day2() {
    // Part 1
    assert_eq!(day2::day2(12, 13, 14, None).0, 2600);
    // Part 2
    assert_eq!(day2::day2(12, 13, 14, None).1, 86036);
}
