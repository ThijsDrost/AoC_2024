use std::fs;
use regex::Regex;

pub fn day3(base_path: &str) {
    let values = fs::read_to_string(base_path.to_owned() + r"\inputs\day3_sample.txt").unwrap();
    let matcher = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(don't\(\))|(do\(\))").unwrap();
    let mut total = 0;
    let mut total2 = 0;
    let mut count = true;
    for value in matcher.captures_iter(&values) {
        if value[0] == "do()".to_string() {
            count = true;
            continue
        }
        if value[0] == "don't()".to_string() {
            count = false;
            continue
        }

        let value1 = value[1].parse::<i32>().unwrap();
        let value2 = value[2].parse::<i32>().unwrap();
        total += value1 * value2;

        if count {
            total2 += value1 * value2;
        }
    }
    println!("Day 3, part 1: {}", total);
    println!("Day 3, part 2: {}", total2);
}