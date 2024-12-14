use std::fs;
use bmp::{Image, Pixel};


fn move_robot(input: &str, steps: u16, size: (i16, i16)) -> (i16, i16) {
    let ((mut p_x, mut p_y), (v_x, v_y)) = parse_robot(input);

    p_x = (p_x + (steps as i16)*v_x).rem_euclid(size.0);
    p_y = (p_y + (steps as i16)*v_y).rem_euclid(size.1);

    (p_x, p_y)
}

fn parse_robot(input: &str) -> ((i16, i16), (i16, i16)) {
    let (pos, vel) = input.split_once(" ").unwrap();
    let (pos, vel) = (pos.replace("p=", ""), vel.replace("v=", ""));
    let (p_x, p_y) = pos.split_once(",").unwrap();
    let (v_x, v_y) = vel.split_once(",").unwrap();

    let (mut p_x, mut p_y) = (p_x.parse::<i16>().unwrap(), p_y.parse::<i16>().unwrap());
    let (v_x, v_y) = (v_x.parse::<i16>().unwrap(), v_y.parse::<i16>().unwrap());
    ((p_x, p_y), (v_x, v_y))
}

fn move_robots(locations: Vec<(i16, i16)>, velocities: Vec<(i16, i16)>, size: (i16, i16), steps: u32, loc_out: String) {
    let mut locations = locations;
    for i in 0..steps {
        for index in 0..velocities.len() {
            locations[index].0 = (locations[index].0 + velocities[index].0).rem_euclid(size.0);
            locations[index].1 = (locations[index].1 + velocities[index].1).rem_euclid(size.1);
        }
        let mut image = Image::new(size.0 as u32, size.1 as u32);
        for loc in locations.iter() {
            image.set_pixel(loc.0 as u32, loc.1 as u32, Pixel::new(255, 255, 255));
        }
        let mut path = loc_out.clone();
        path.push_str("\\");
        path.push_str(&i.to_string());
        path.push_str(".bmp");
        image.save(path).unwrap();
    }
}

pub fn day14(base_path: &str, real: bool) -> (u64, u64) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day14.txt"
        } else {
            base_path.to_owned() + r"\inputs\day14_sample.txt"
        }
    };
    let start = fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = start.lines().collect();

    let map_size = (101, 103);
    let locations: Vec<(i16, i16)> = lines.iter().map(|x| {
        move_robot(x, 100, map_size.clone())
    }).collect();
    let mut values = vec![0; 4];

    for x in locations.iter() {
        if x.0 < map_size.0/2 {
            if x.1 < map_size.1/2 {
                values[0] += 1;
            }
            else if x.1 > map_size.1/2 {
                values[1] += 1;
            }
        }
        else if x.0 > map_size.0/2 {
            if x.1 < map_size.1/2 {
                values[2] += 1;
            }
            else if x.1 > map_size.1/2 {
                values[3] += 1;
            }
        }
    }

    // let (pos, vel): (Vec<(i16, i16)>, Vec<(i16, i16)>) = lines.iter().map(|x| {
    //     parse_robot(x)
    // }).unzip();
    // let path = r"C:\Users\drost\RustroverProjects\AoC_2024\day14_images";
    // move_robots(pos, vel, map_size.clone(), 10_000, path.to_string());


    (values[0]*values[1]*values[2]*values[3], 6243)
}