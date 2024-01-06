#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fs;

pub(crate) fn day3(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day3/data/main.txt").unwrap());
    let data = data.lines().collect::<Vec<&str>>();
    let re = regex::Regex::new(r"\d+").unwrap();
    let symbols = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];

    let len = data.len();
    let mut valid_nums: Vec<i32> = vec![];
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for (y, line) in data.iter().enumerate() {
        // find all numbers in string
        let nums_match = re.find_iter(line).collect::<Vec<_>>();
        for num_match in nums_match {
            let orig_start = num_match.start();
            let mut start = orig_start;
            let orig_end = num_match.end();
            let mut end = orig_end;

            // include left
            start = start.saturating_sub(1);
            // include right
            if end < line.len() {
                end += 1;
            }

            let mut check_chars = |chars: Vec<char>, x_offset: usize, y: usize| -> bool {
                let mut valid = false;
                for (x, char) in chars.iter().enumerate() {
                    for symbol in symbols {
                        if char == &symbol {
                            let num = line[orig_start..orig_end].parse::<i32>().unwrap();
                            valid_nums.push(num);
                            valid = true;
                            if char == &'*' {
                                let gear = gears.entry((x + x_offset, y)).or_default();
                                gear.push(num);
                            }
                        }
                    }
                }
                valid
            };

            // check upper
            if y > 0 {
                let chars = data[y - 1].chars().collect::<Vec<char>>();
                let chars = chars[start..end].to_vec();
                if check_chars(chars, start, y - 1) {
                    continue;
                }
                // for char in chars {
                //     if symbols.contains(&char) {
                //         valid_nums.push(line[start..end].parse::<i32>().unwrap());
                //         continue;
                //     }
                // }
            }
            // check lower
            if y < len - 1 {
                let chars = data[y + 1].chars().collect::<Vec<char>>();
                let chars = chars[start..end].to_vec();
                // for char in chars {
                //     if symbols.contains(&char) {
                //         valid_nums.push(line[start..end].parse::<i32>().unwrap());
                //         continue;
                //     }
                // }
                if check_chars(chars, start, y + 1) {
                    continue;
                }
            }
            // check current
            let chars = line.chars().collect::<Vec<char>>();
            let chars = chars[start..end].to_vec();
            // for char in chars {
            //     if symbols.contains(&char) {
            //         valid_nums.push(line[start..end].parse::<i32>().unwrap());
            //         continue;
            //     }
            // }
            if check_chars(chars, start, y) {
                continue;
            }

            // // check left
            // if start > 0 {
            //     let left_pos = start - 1;
            //     if check_column(&data, &symbols, left_pos, i) {
            //         valid_nums.push(line[start..right_pos].parse::<i32>().unwrap());
            //         continue;
            //     }
            // }
            // // check right
            // if right_pos < line.len() && check_column(&data, &symbols, right_pos, i) {
            //     valid_nums.push(line[start..right_pos].parse::<i32>().unwrap());
            //     continue;
            // }
            // // check upper and lower
            // if check_column(&data, &symbols, start, i) {
            //     valid_nums.push(line[start..right_pos].parse::<i32>().unwrap());
            //     continue;
            // }
        }
    }

    // sum valid nums
    let mut sum = 0;
    for num in valid_nums {
        sum += num;
    }

    // get gear ratios
    let mut ratios = 0;
    for gear in gears.values() {
        if gear.len() == 2 {
            ratios += gear[0] * gear[1];
        }
    }

    (sum, ratios)
}
