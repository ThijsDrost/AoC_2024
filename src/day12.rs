use std::collections::{HashSet, VecDeque};
use std::fs;

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
    let mut total_been: HashSet<(usize, usize)> = HashSet::new();
    let mut total_val = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if total_been.insert((i, j)) {
                let mut queue = VecDeque::new();
                let mut been: HashSet<(usize, usize)> = HashSet::new();
                been.insert((i, j));
                let mut total_area = 1;
                let mut total_edge = 0;
                queue.push_back((i as i32, j as i32));
                while !queue.is_empty() {
                    let (i, j) = queue.pop_front().unwrap();
                    for direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let new_loc = (i + direction.0, j + direction.1);
                        if been.contains(&(new_loc.0 as usize, new_loc.1 as usize)){
                            continue;
                        }

                        if 0 <= new_loc.0 && new_loc.0 < map.len() as i32
                            && 0 <= new_loc.1 && new_loc.1 < map[0].len() as i32
                            && map[i as usize][j as usize] == map[new_loc.0 as usize][new_loc.1 as usize]
                        {
                                queue.push_back(new_loc);
                                been.insert((new_loc.0 as usize, new_loc.1 as usize));
                                total_area += 1;
                        }
                        else {
                            total_edge += 1;
                        }
                    }
                }
                total_been.extend(&been);
                // println!("{:?} {:?} {:?}", map[i][j], total_area, total_edge);
                total_val += total_area*total_edge;
            }
        }
    }

    (total_val, 0)
}
