use std::fs;

pub fn day4(base_path: &str) {
    let values = fs::read_to_string(base_path.to_owned() + r"\inputs\day4_sample.txt").unwrap();
    let values = values.lines()
        .map(|l| l.chars().map(|c|
            match c {
                'X' => 1,
                'M' => 2,
                'A' => 3,
                'S' => 4,
                _ => panic!("Unexpected character!")
            }
        ).collect::<Vec<u8>>()
        ).collect::<Vec<Vec<u8>>>();
    let directions: Vec<[i32; 2]> = vec![[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [-1, 1], [1, -1], [-1, -1]];
    let mut total = 0;
    for i in 0..(values.len() as i32) {
        for j in 0..(values[i as usize].len() as i32) {
            if values[i as usize][j as usize] != 1 {
                continue;
            }
            for dir in &directions {
                if (!(0..(values.len() as i32)).contains(&(i + 3 * dir[0])))
                    || (!(0..(values[i as usize].len() as i32)).contains(&(j + 3 * dir[1]))){
                    continue;
                }
                if values[(i+dir[0]) as usize][(j+dir[1]) as usize] != 2
                    || values[(i+2*dir[0]) as usize][(j+2*dir[1]) as usize] != 3
                    || values[(i+3*dir[0]) as usize][(j+3*dir[1]) as usize] != 4 {
                    continue
                }
                total += 1;
            }
        }
    }

    let mut total2 = 0;
    for i in 1..(values.len() -1) {
        for j in 1..(values[i].len() -1) {
            if values[i][j] == 3 {
                if ((values[i-1][j-1] == 2 && values[i+1][j+1] == 4)
                    || (values[i-1][j-1] == 4 && values[i+1][j+1] == 2))
                    && ((values[i-1][j+1] == 2 && values[i+1][j-1] == 4)
                    || (values[i-1][j+1] == 4 && values[i+1][j-1] == 2)){
                    total2 += 1;
                }
            }
        }
    }

    println!("Day 4 part 1: {}", total);
    println!("Day 4 part 2: {}", total2);
}