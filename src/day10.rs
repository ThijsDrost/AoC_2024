use std::collections::{HashSet, VecDeque};
use std::fs;
use rayon::prelude::*;

pub fn solution(start: (usize, usize), map: &Vec<Vec<u32>>) -> (usize, usize) {
    let mut queue: VecDeque<(usize, usize, u32, bool)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0, true));
    let mut been: HashSet<(i32, i32)> = HashSet::new();
    let mut values = 0;
    let mut values2 = 0;
    while queue.len() != 0 {
        let mut entry = queue.pop_front().unwrap();
        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_loc = ( entry.0 as i32 + direction.0, entry.1 as i32 + direction.1 );

            if (0..map.len() as i32).contains(&new_loc.0)
                && (0..map[0].len() as i32).contains(&new_loc.1)
                && map[new_loc.0 as usize][new_loc.1 as usize] == (entry.2 + 1)
            {
                if !been.insert(new_loc) {
                    entry.3 = false;
                }
                if entry.2 + 1 == 9 {
                    values2 += 1;
                    if entry.3 {
                        values += 1;
                    }
                }
                else {
                    queue.push_back((new_loc.0 as usize, new_loc.1 as usize, entry.2 + 1, entry.3));
                }
            }
        }
    }
    (values, values2)
}


pub fn day10(base_path: &str, real: bool) -> (usize, usize) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day10.txt"
        } else {
            base_path.to_owned() + r"\inputs\day10_sample.txt"
        }
    };
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let values = fs::read_to_string(&path).unwrap();
    let map = values.lines().enumerate().map(|(i, x)| {
            x.chars().enumerate().map(|(j, y)| {
                let val = y.to_digit(10).unwrap();
                if val == 0 {
                    starts.push((i, j));
                }
                val
            }).collect()
        }).collect::<Vec<Vec<u32>>>();

    let total: (Vec<_>, Vec<_>) = starts.par_iter().map(|start|
        solution(*start, &map)
    ).unzip();
    let values = (total.0.iter().sum::<usize>(), total.1.iter().sum::<usize>());

    values
}