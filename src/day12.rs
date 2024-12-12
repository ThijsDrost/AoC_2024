use std::fs;
use std::collections::VecDeque;
use rustc_hash::FxHashSet as HashSet;

pub fn day12(base_path: &str, real: bool) -> (u64, u64) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day12.txt"
        } else {
            base_path.to_owned() + r"\inputs\day12_sample.txt"
        }
    };
    let start = fs::read_to_string(&path).unwrap();
    let map: Vec<Vec<char>> = start.lines().map(|l| l.chars().collect()).collect();
    let mut total_been: HashSet<(i32, i32)> = HashSet::default();
    let mut total_val = 0;
    let mut total_val2 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !total_been.insert((i as i32, j as i32)) {
                continue;
            }
            let mut queue = VecDeque::new();
            let mut been: HashSet<(i32, i32)> = HashSet::default();
            let mut total_area = 1;
            let mut total_edge = 0;
            been.insert((i as i32, j as i32));
            queue.push_back((i as i32, j as i32));
            while !queue.is_empty() {
                let (i2, j2) = queue.pop_front().unwrap();
                for direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_loc = (i2 + direction.0, j2 + direction.1);
                    if been.contains(&(new_loc.0, new_loc.1)){
                        continue;
                    }

                    if 0 <= new_loc.0 && new_loc.0 < map.len() as i32
                        && 0 <= new_loc.1 && new_loc.1 < map[0].len() as i32
                        && map[i2 as usize][j2 as usize] == map[new_loc.0 as usize][new_loc.1 as usize]
                    {
                        queue.push_back(new_loc);
                        been.insert((new_loc.0, new_loc.1));
                        total_area += 1;
                    }
                    else {
                        total_edge += 1;
                    }
                }
            }
            // Part2: as many corners as sides
            let mut corners = 0;
            for square in been.iter() {
                let up = (square.0 - 1, square.1);
                let down = (square.0 + 1, square.1);
                let left = (square.0, square.1 - 1);
                let right = (square.0 , square.1 + 1);

                if !been.contains(&up) && !been.contains(&left) {
                    corners += 1;
                }
                if !been.contains(&up) && !been.contains(&right) {
                    corners += 1;
                }
                if !been.contains(&down) && !been.contains(&left) {
                    corners += 1;
                }
                if !been.contains(&down) && !been.contains(&right) {
                    corners += 1;
                }

                if been.contains(&up) && been.contains(&left) && !been.contains(&(square.0 - 1, square.1 - 1)) {
                    corners += 1;
                }
                if been.contains(&up) && been.contains(&right) && !been.contains(&(square.0 - 1, square.1 + 1)) {
                    corners += 1;
                }
                if been.contains(&down) && been.contains(&left) && !been.contains(&(square.0 + 1, square.1 - 1)) {
                    corners += 1;
                }
                if been.contains(&down) && been.contains(&right) && !been.contains(&(square.0 + 1, square.1 + 1)) {
                    corners += 1;
                }
            }
            total_been.extend(&been);
            total_val += total_area*total_edge;
            total_val2 += total_area*corners;
        }
    }
    (total_val, total_val2)
}
