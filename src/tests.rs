#[cfg(test)]
mod aoc_tests {
    use std::fs;
    use crate::{day1, day10, day2, day3, day4, day5, day6, day7, day8, day9};

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

    #[test]
    fn test_day2_test_data() {
        let test_data = fs::read_to_string("src/test_data/2.txt").unwrap();

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

    #[test]
    fn test_day3_test_data() {
        let test_data = fs::read_to_string("src/test_data/3.txt").unwrap();

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

    #[test]
    fn test_day4_test_data() {
        let test_data = fs::read_to_string("src/test_data/4.txt").unwrap();

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

    #[test]
    fn test_day5_test_data() {
        let test_data = fs::read_to_string("src/test_data/5.txt").unwrap();

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

    #[tokio::test]
    async fn test_day6_test_data() {
        let test_data = fs::read_to_string("src/test_data/6.txt").unwrap();

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

    #[tokio::test]
    async fn test_day7_test_data() {
        let test_data = fs::read_to_string("src/test_data/7.txt").unwrap();

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

    #[tokio::test]
    async fn test_day8_test_data() {
        let test_data = fs::read_to_string("src/test_data/8.txt").unwrap();
        let test_data_2 = fs::read_to_string("src/test_data/8_2.txt").unwrap();

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

    #[tokio::test]
    async fn test_day9_test_data() {
        let test_data = fs::read_to_string("src/test_data/9.txt").unwrap();
        let (part1, part2) = day9::day9(Some(test_data)).await;

        // Part 1
        assert_eq!(part1, 114);
        // Part 2
        assert_eq!(part2, 2);
    }

    #[tokio::test]
    async fn test_day9() {
        let (part1, part2) = day9::day9(None).await;

        // Part 1
        assert_eq!(part1, 1853145119);
        // Part 2
        assert_eq!(part2, 923);
    }

    #[tokio::test]
    async fn test_day10_test_data() {
        let test_data = fs::read_to_string("src/test_data/10.txt").unwrap();
        let result = day10::day10(Some(test_data)).await;

        // Part 1
        assert_eq!(result.0, 80);
        // Part 2
        assert_eq!(result.1, 10);
    }

    #[tokio::test]
    async fn test_day10() {
        let result = day10::day10(None).await;

        // Part 1
        assert_eq!(result.0, 6599);
        // Part 2
        assert_eq!(result.1, 477);
    }
}
