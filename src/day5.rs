use std::fs;
use std::collections::HashMap;

pub fn day5(base_path: &str, real: bool) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day5.txt"
        } else {
            base_path.to_owned() + r"\inputs\day5_sample.txt"
        }
    };
    let values = fs::read_to_string(&path).unwrap();
    let mut mappert = HashMap::<i32, Vec<i32>>::new();
    let mut first = true;
    let mut total = 0;
    let mut total2 = 0;
    for line in values.lines() {
        if first {
            if line == "" {
                first = false;
                continue;
            }
            let (value1, value2) = line.split_once('|').unwrap();
            let value1 = value1.parse::<i32>().unwrap();
            let value2 = value2.parse::<i32>().unwrap();
            if let Some(val) = mappert.get_mut(&value2) {
                val.push(value1);
            }
            else {
                mappert.insert(value2, vec![value1]);
            }
        }
        else {
            let mut values = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut correct = true;
            'loopert: loop {
                let mut settert = HashMap::<i32, usize>::new();
                for index in 0..values.len() {
                    if let Some(value) = mappert.get(&values[index]) {
                        for val in value.iter() {
                            settert.insert(*val, index);
                        }
                    }
                    if let Some(index2) = settert.get(&values[index]) {
                        correct = false;
                        values.swap(index, *index2);
                        continue 'loopert;
                    }
                }
                break
            }
            if correct {
                total += values[values.len() / 2];
            }
            else {
                total2 += values[values.len() / 2];
            }
        }
    }
    println!("Day 5, part 1: {}", total);
    println!("Day 5, part 2: {}", total2);
}