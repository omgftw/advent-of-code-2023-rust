use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::read;
use std::io::BufRead;

lazy_static! {
    static ref WORDS: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

pub(crate) fn words_to_num(string: &str) -> String {
    let mut result = String::new();
    for (index, char) in string.char_indices() {
        let remainder = &string[index..];
        for (word, num) in WORDS.iter() {
            if char.is_ascii_digit() {
                result.push(char);
                break;
            }
            if remainder.starts_with(word) {
                result.push_str(&num.to_string());
            }
        }
    }

    result
}

pub(crate) fn process_lines(mut line: String, part2: bool) -> i32 {
        if part2 {
            line = words_to_num(&line);
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
        let last = last.unwrap();
        first.push_str(&last);
        first.parse::<i32>().unwrap()
}

pub(crate) fn day1(part2: bool) -> i32 {
    let data = read("src/data/1-1.txt");
    // let words: HashMap<&str, i32> = HashMap::from([
    //     ("one", 1),
    //     ("two", 2),
    //     ("three", 3),
    //     ("four", 4),
    //     ("five", 5),
    //     ("six", 6),
    //     ("seven", 7),
    //     ("eight", 8),
    //     ("nine", 9),
    // ]);

    let mut nums: Vec<i32> = vec![];
    for line in data.unwrap().lines() {
        let line = line.unwrap();
        let num = process_lines(line, part2);
        nums.push(num);
    }

    // sum nums
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}
