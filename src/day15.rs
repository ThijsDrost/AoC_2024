use std::fs;


fn print_map(map: &Vec<Vec<u8>>, position: &(usize, usize)) {
    for (i, line) in map.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            let val = match char {
                0 => '#',
                1 => '.',
                2 => 'O',
                _ => panic!("unexpected character: {:?}", char),
            };
            if (i == position.0 && j == position.1) {
                print!("@");
            }
            else {
                print!("{}", val);
            }
        }
        println!();
    }
}


pub fn day15(base_path: &str, real: bool) -> (usize, u64) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day15.txt"
        } else {
            base_path.to_owned() + r"\inputs\day15_sample.txt"
        }
    };
    let start = fs::read_to_string(&path).unwrap();
    let mut position: (usize, usize) = (0, 0);
    let mut directions: Vec<(i32, i32)> = Vec::new();
    let mut begin = true;
    let mut map: Vec<Vec<u8>> = Vec::new();
    for (i, line) in start.lines().enumerate() {
        if line == "" {
            begin = false;
            continue;
        }
        if !begin {
            let these_directions: Vec<(i32, i32)> = line.chars().map(| c | {
                match c {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    '<' => (0, -1),
                    'v' => (1, 0),
                    _ => panic!("unexpected character: {:?}", c),
                }
            }).collect();
            directions.extend(these_directions);
        }
        else {
            let vec: Vec<u8> = line.chars().enumerate().map(| (j, c)| {
                match c {
                    '#' => 0,
                    '.' => 1,
                    'O' => 2,
                    '@' => {
                        position = (i, j);
                        1
                    }
                    _ => panic!("unexpected character: {:?}", c),
                }
            }).collect();
            map.push(vec);
        }
    }

    for dir in directions {
        let mut new_position = ((position.0 as i32 + dir.0) as usize, (position.1 as i32 + dir.1) as usize);
        if map[new_position.0][new_position.1] == 1 {
            position = new_position;
        }
        else if map[new_position.0][new_position.1] == 0 {
            continue;
        }
        else if map[new_position.0][new_position.1] == 2 {
            let mut possible = false;

            let mut newest_position = ((new_position.0 as i32 + dir.0) as usize, (new_position.1 as i32 + dir.1) as usize);
            while map[newest_position.0][newest_position.1] != 0 {
                if map[newest_position.0][newest_position.1] == 1 {
                    possible = true;
                    break;
                }
                newest_position = ((newest_position.0 as i32 + dir.0) as usize, (newest_position.1 as i32 + dir.1) as usize);
            }
            if possible {
                map[newest_position.0][newest_position.1] = 2;
                map[new_position.0][new_position.1] = 1;
                position = new_position;
            }
        }
    }
    print_map(&map, &position);

    let mut total  = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 2 {
                total += 100*i + j;
            }
        }
    }

    (total, 0)
}