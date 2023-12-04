use std::collections::HashMap;
use std::fs::read;
use std::io::BufRead;

fn words_to_num<>(string: &str, map: &HashMap<&str, i32>) -> String {
    let mut result = String::new();
    for (index, char) in string.char_indices() {
        let remainder = &string[index..];
        for (word, num) in map {
            if char.is_ascii_digit() {
                result.push(char);
            }
            if remainder.starts_with(word) {
                result.push_str(&num.to_string());
            }
        }
    }

    result
}

pub(crate) fn day1(part2: bool) -> i32 {
    let data = read("src/data/1-1.txt");
    let words: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut nums: Vec<i32> = vec![];
    for line in data.unwrap().lines() {
        let mut line = line.unwrap();
        if part2 {
            line = words_to_num(&line, &words);
        }
        let mut first: Option<String> = None;
        let mut last: Option<String> = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c.to_string());
                }
                last = Some(c.to_string());
            }
        }
        let mut first = first.unwrap();
        first.push_str(&(last.unwrap()));
        nums.push(first.parse::<i32>().unwrap());
    }

    // sum nums
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}