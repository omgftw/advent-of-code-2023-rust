#[cfg(test)]
mod aoc_tests {
    use crate::{day1, day2, day3, day4, day5, day6};

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
        let test_data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            .to_string();

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
        let test_data = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .to_string();

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
        let test_data = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

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
        let test_data = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

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
        let test_data = r#"Time:      7  15   30
Distance:  9  40  200"#;

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
}
