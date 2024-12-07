use std::fs;
use rayon::prelude::*;

fn concat(a: i64, b: i64) -> i64 {
    let power = (b as f64).log10().floor() as u32;
    a * 10_i64.pow(power+1) + b
}

fn concat2(a: i64, b: i64) -> i64 {
    let mut uno = a.to_string();
    let dos = b.to_string();
    uno.push_str(&dos);
    uno.parse::<i64>().unwrap()
}


fn part1(target: i64, current: i64, index: usize, values: &Vec<i64>) -> bool {
    let val = values[index];
    let times_val = current * val;
    let plus_val = current + val;

    if (index + 1) == values.len() {
        return (times_val == target) || (plus_val == target)
    }
    let bool1 = {
        if times_val <= target {
            part1(target, times_val, index + 1, values)
        }
        else {
            false
        }
    };
    let bool2 = {
        if plus_val <= target {
            part1(target, plus_val, index + 1, values)
        }
        else {
            false
        }
    };
    bool1 || bool2
}

fn part2(target: i64, current: i64, index: usize, values: &Vec<i64>) -> bool {
    let val = values[index];

    let times_val = current * val;
    let plus_val = current + val;
    let concat_val = {
        if current == 0 {
            -1
        }
        else{
            concat(current, values[index])
        }
    };

    if (index + 1) == values.len() {
        return (times_val == target) || (plus_val == target) || (concat_val == target);
    }
    let bool1 = {
        if times_val <= target {
            part2(target, times_val, index + 1, values)
        }
        else {
            false
        }
    };
    let bool2 = {
        if plus_val <= target {
            part2(target, plus_val, index + 1, values)
        }
        else {
            false
        }
    };
    let bool3 = {
        if concat_val <= target && concat_val != -1 {
            part2(target, concat_val, index + 1, values)
        }
        else {
            false
        }
    };
    bool1 || bool2 || bool3
}

pub fn day7(base_path: &str, real: bool) -> (i64, i64) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day7.txt"
        } else {
            base_path.to_owned() + r"\inputs\day7_sample.txt"
        }
    };
    let values = fs::read_to_string(&path).unwrap();
    let values = values
        .lines()
        .collect::<Vec<&str>>();

    let values1: i64 = values
        .par_iter()
        .map(|x| x.split_once(":").unwrap())
        .map(|(uno, dos)| (uno.parse::<i64>().unwrap(), dos.split_whitespace()
            .map(|val| val.parse::<i64>().unwrap()).collect::<Vec<i64>>()))
        .map(|(x, y) | {
            if part1(x, 0, 0, &y) {
                x
            }
            else {
                0
            }
        })
        .sum();

    let values2: i64 = values
        .par_iter()
        .map(|x| x.split_once(":").unwrap())
        .map(|(uno, dos)| (uno.parse::<i64>().unwrap(), dos.split_whitespace()
            .map(|val| val.parse::<i64>().unwrap()).collect::<Vec<i64>>()))
        .map(|(x, y) | {
            if part2(x, 0, 0, &y) {
                x
            }
            else {
                0
            }
        })
        .sum();
    (values1, values2)
}