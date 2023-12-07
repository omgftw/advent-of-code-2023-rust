#[cfg(test)]
mod aoc_tests {
    use crate::{day1, day2, day3};

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
        assert_eq!(day1::process_lines("1abc2", false), 12);
        assert_eq!(day1::process_lines("pqr3stu8vwx", false), 38);
        assert_eq!(day1::process_lines("a1b2c3d4e5f", false), 15);
        assert_eq!(day1::process_lines("treb7uchet", false), 77);

        assert_eq!(day1::day1(false), 54990);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(day1::process_lines("two1nine", true), 29);
        assert_eq!(day1::process_lines("eightwothree", true), 83);
        assert_eq!(day1::process_lines("abcone2threexyz", true), 13);
        assert_eq!(day1::process_lines("xtwone3four", true), 24);
        assert_eq!(day1::process_lines("4nineeightseven2", true), 42);
        assert_eq!(day1::process_lines("zoneight234", true), 14);
        assert_eq!(day1::process_lines("7pqrstsixteen", true), 76);

        assert_eq!(day1::process_lines("oneight", true), 18);

        assert_eq!(day1::day1(true), 54473);
    }

    #[test]
    fn test_day2_part1() {
        let test_data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.to_string();
        assert_eq!(day2::day2(12, 13, 14, Some(test_data)).0, 8);

        assert_eq!(day2::day2(12, 13, 14, None).0, 2600);
    }

    #[test]
    fn test_day2_part2() {
        let test_data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.to_string();
        assert_eq!(day2::day2(12, 13, 14, Some(test_data)).1, 2286);

        assert_eq!(day2::day2(12, 13, 14, None).1, 86036);
    }

    #[test]
    fn test_day3_part1() {
        let test_data = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#.to_string();
        let result = day3::day3(Some(test_data));
        assert_eq!(result.0, 4361);
        assert_eq!(result.1, 467835);
    }

    #[test]
    fn test_day3_part2() {
        let result = day3::day3(None);
        assert_eq!(result.0, 525911);
        assert_eq!(result.1, 75805607);
    }
}
