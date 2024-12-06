use std::fs;
use std::collections::HashMap;
use rayon::prelude::*;

pub fn correct_function(mut values: Vec<i32>, mappert: &HashMap::<i32, Vec<i32>>) -> (i32, i32) {
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
        (values[values.len() / 2], 0)
    }
    else {
        (0,  values[values.len() / 2])
    }
}


pub fn day5(base_path: &str, real: bool) -> (i32, i32) {
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

    let mut inputs: Vec<Vec<i32>> = Vec::new();

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
            inputs.push(line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }
    }
    let results = inputs.par_iter().map(
        |x| correct_function(x.clone(), &mappert)
    ).reduce(|| (0, 0), |(x, x1), (y, y1)| (x + y, x1 + y1));

    results
}