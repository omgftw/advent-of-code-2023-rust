#[cfg(test)]
mod aoc_tests {
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
    fn test_day1_par1() {
        assert_eq!(day1::process_lines("1abc2".to_string(), false), 12);
        assert_eq!(day1::process_lines("pqr3stu8vwx".to_string(), false), 38);
        assert_eq!(day1::process_lines("a1b2c3d4e5f".to_string(), false), 15);
        assert_eq!(day1::process_lines("treb7uchet".to_string(), false), 77);

        assert_eq!(day1::day1(false), 54990);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(day1::process_lines("two1nine".to_string(), true), 29);
        assert_eq!(day1::process_lines("eightwothree".to_string(), true), 83);
        assert_eq!(day1::process_lines("abcone2threexyz".to_string(), true), 13);
        assert_eq!(day1::process_lines("xtwone3four".to_string(), true), 24);
        assert_eq!(day1::process_lines("4nineeightseven2".to_string(), true), 42);
        assert_eq!(day1::process_lines("zoneight234".to_string(), true), 14);
        assert_eq!(day1::process_lines("7pqrstsixteen".to_string(), true), 76);

        assert_eq!(day1::process_lines("oneight".to_string(), true), 18);

        assert_eq!(day1::day1(true), 54473);
    }
}
