use std::fs;

fn test_values(value1: i32, value2: i32, expected_sign: i32) -> bool {
    let diff = value2 - value1;
    if diff.signum() != expected_sign || diff.abs() > 3 {
        return false
    }
    true
}

fn tester(values: &Vec<i32>) -> Option<usize>{
    let diff = values[1] - values[0];
    if diff == 0 || diff.abs() > 3 {
        return Some(0)
    }
    let sign = diff.signum();
    for index in 1..(values.len() -1) {
        if !test_values(values[index], values[index+1], sign) {
            return Some(index)
        }
    }
    None
}

pub fn day2(){
    let values = fs::read_to_string(r"C:\Users\drost\RustroverProjects\AoC\inputs\day2_sample.txt").unwrap();
    let values: Vec<Vec<i32>> = values.lines().map(|line | {
            line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        })
        .collect();
    let res: i32 = values.iter()
        .filter_map(|values | {
            if tester(values).is_some() {
                return None
            }
            return Some(1)
        }).sum();
    println!("Day 2, part 1: {}", res);

    let res: i32 = values.iter()
        .filter_map(|values | {
            if let Some(index) = tester(values) {
                if index == 1 {
                    let mut values0 = values.clone();
                    values0.remove(0);
                    if tester(&values0).is_none() {
                        return Some(1)
                    }
                }
                let mut values1 = values.clone();
                values1.remove(index);
                let mut values2 = values.clone();
                values2.remove(index+1);
                if tester(&values1).is_some() && tester(&values2).is_some() {
                    return None
                }
            }
            return Some(1)
        }).sum();
    println!("Day 2, part 2: {}", res);
}