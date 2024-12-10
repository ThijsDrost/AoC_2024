use std::fs;
use rayon::prelude::*;
use rustc_hash::FxHashSet;


pub fn check_loop(start_position: (usize, usize), block_position: (usize, usize), obstacle_map: Vec<Vec<bool>>) -> Option<()> {
    let mut position = start_position.clone();
    let mut been: FxHashSet<(usize, usize, i32, i32)> = FxHashSet::default();
    let mut direction: (i32, i32) = (-1, 0);
    let mut this_obstacle_map = obstacle_map.clone();
    this_obstacle_map[block_position.0][block_position.1] = false;

    'outer: loop {
        been.insert((position.0, position.1, direction.0, direction.1));
        while (0..this_obstacle_map.len()).contains(&position.0)
            && (0..this_obstacle_map[0].len()).contains(&position.1) {
            if !this_obstacle_map[position.0][position.1] {
                position = ((position.0 as i32 - direction.0) as usize, (position.1 as i32 - direction.1) as usize);
                direction = (direction.1, -1 * direction.0);
                if been.contains(&(position.0, position.1, direction.0, direction.1)) {
                    return Some(());
                }
                continue 'outer;
            }
            position = ((position.0 as i32 + direction.0) as usize, (position.1 as i32 + direction.1) as usize);
        }
        return None;
    }
}


pub fn day6(base_path: &str, real: bool) -> (i32, i32) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day6.txt"
        } else {
            base_path.to_owned() + r"\inputs\day6_sample.txt"
        }
    };
    let values = fs::read_to_string(&path).unwrap();
    let mut start_position = (0, 0);
    let obstacle_map = values.lines()
        .enumerate()
        .map(|(index, line)| line.chars()
            .enumerate()
            .map(|(index2, x)|
                match x {
                    '.' => true,
                    '#' => false,
                    '^' => {
                        start_position = (index, index2);
                        true
                        },
                    _ => panic!("Unexpected character"),
                }
            ).collect::<Vec<bool>>()
        ).collect::<Vec<Vec<bool>>>();

    let mut position = start_position.clone();
    let mut been: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut visited: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut direction = (-1, 0);
    'outer: loop {
        been.insert(position);
        while (0..obstacle_map.len()).contains(&position.0)
            && (0..obstacle_map[0].len()).contains(&position.1) {
            if !obstacle_map[position.0][position.1] {
                position = ((position.0 as i32 - direction.0) as usize, (position.1 as i32 - direction.1) as usize);
                direction = (direction.1, -1*direction.0);
                continue 'outer;
            }
            if !visited.contains(&position) {
                visited.insert(position);
            }
            position = ((position.0 as i32 + direction.0) as usize, (position.1 as i32 + direction.1) as usize);
        }
        break 'outer;
    }

    // Put an obstacle on each visited location and test whether a loop is formed
    let total = visited.clone().par_iter().filter_map(
        |x| check_loop(start_position, *x, obstacle_map.clone())
    ).count();

    (visited.len() as i32, total as i32)
}