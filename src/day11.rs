use std::fs;
use rustc_hash::FxHashMap as HashMap;

fn split_number(input: u64) -> (u64, u64) {
    let string = input.to_string();
    let values = string.split_at(string.len()/2);
    (values.0.parse().unwrap(), values.1.parse().unwrap())
}

#[inline]
fn insert_value(map: &mut HashMap<u64, u64>, key: u64, num: u64) {
    if let Some(val) = map.get_mut(&key) {
        *val += num;
    }
    else {
        map.insert(key, num);
    }
}


pub fn day11(base_path: &str, real: bool) -> (u64, u64) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day11.txt"
        } else {
            base_path.to_owned() + r"\inputs\day11_sample.txt"
        }
    };
    let start = fs::read_to_string(&path).unwrap();
    let mut last_mapping: HashMap<u64, u64> = HashMap::default();
    for entry in start.split_whitespace() {
        let value = entry.parse::<u64>().unwrap();
        insert_value(&mut last_mapping, value, 1);
    }
    let mut new_mapping: HashMap<u64, u64>;
    let mut total1 = 0;
    for index in 1..76 {
        new_mapping = HashMap::default();
        for key in last_mapping.keys() {
            if *key == 0 {
                insert_value(&mut new_mapping, 1, last_mapping[key]);
            }
            else if (key.ilog10() + 1) % 2 == 0 {
                let values = split_number(*key);
                insert_value(&mut new_mapping, values.0, last_mapping[key]);
                insert_value(&mut new_mapping, values.1, last_mapping[key]);
            }
            else {
                insert_value(&mut new_mapping, 2024*key, last_mapping[key]);
            }
        }
        last_mapping = new_mapping;
        if index == 25 {
            total1 = last_mapping.iter().map(|hm| hm.1).sum::<u64>();
        }
    }

    let total2 = last_mapping.iter().map(|hm| hm.1).sum::<u64>();
    (total1, total2)
}