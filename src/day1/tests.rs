use crate::day1;

#[test]
fn test_day1_words_to_num() {
    assert_eq!(day1::words_to_num("two1nine"), "219");
    assert_eq!(day1::words_to_num("eightwothree"), "823");
    assert_eq!(day1::words_to_num("abcone2threexyz"), "123");
    assert_eq!(day1::words_to_num("xtwone3four"), "2134");
    assert_eq!(day1::words_to_num("4nineeightseven2"), "49872");
    assert_eq!(day1::words_to_num("zoneight234"), "18234");
    assert_eq!(day1::words_to_num("7pqrstsixteen"), "76");
}

#[test]
fn test_day1_test_data() {
    // Part 1
    assert_eq!(day1::process_lines("1abc2", false), 12);
    assert_eq!(day1::process_lines("pqr3stu8vwx", false), 38);
    assert_eq!(day1::process_lines("a1b2c3d4e5f", false), 15);
    assert_eq!(day1::process_lines("treb7uchet", false), 77);

    // Part 2
    assert_eq!(day1::process_lines("two1nine", true), 29);
    assert_eq!(day1::process_lines("eightwothree", true), 83);
    assert_eq!(day1::process_lines("abcone2threexyz", true), 13);
    assert_eq!(day1::process_lines("xtwone3four", true), 24);
    assert_eq!(day1::process_lines("4nineeightseven2", true), 42);
    assert_eq!(day1::process_lines("zoneight234", true), 14);
    assert_eq!(day1::process_lines("7pqrstsixteen", true), 76);
    // Additional test data for undocumented cases
    assert_eq!(day1::process_lines("oneight", true), 18);
}

#[test]
fn test_day1() {
    // Part 1
    assert_eq!(day1::day1(false), 54990);
    // Part 2
    assert_eq!(day1::day1(true), 54473);
}
