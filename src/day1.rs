use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

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

pub(crate) fn process_lines(line: &str, part2: bool) -> i32 {
        let output: String;
        if part2 {
            output = words_to_num(&line);
        } else {
            output = line.to_string();
        }
        let mut first: Option<String> = None;
        let mut last: Option<String> = None;
        for c in output.chars() {
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
    let data = fs::read_to_string("src/data/1.txt").unwrap();

    let mut nums: Vec<i32> = vec![];
    for line in data.lines() {
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
