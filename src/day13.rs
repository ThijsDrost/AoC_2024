use std::fs;
use nalgebra;

pub fn day13(base_path: &str, real: bool) -> (u64, u64) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day13.txt"
        } else {
            base_path.to_owned() + r"\inputs\day13_sample.txt"
        }
    };
    let start = fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = start.lines().collect();
    let pockets = lines.chunks(4);
    let results: (Vec<u64>, Vec<u64>) = pockets.map(|p| button_presses(p)).unzip();

    (results.0.iter().sum(), results.1.iter().sum())
}


fn button_presses(input: &[&str]) -> (u64, u64) {
    fn parse_line(input: &str) -> (usize, usize) {
        let line1 = input.split_once(":").unwrap().1;
        let values = line1.split_once(",").unwrap();
        let x1 = values.0.split_once("+").unwrap().1.parse::<usize>().unwrap();
        let y1 = values.1.split_once("+").unwrap().1.parse::<usize>().unwrap();
        (x1, y1)
    }
    let (x1, y1) = parse_line(input[0]);
    let (x2, y2) = parse_line(input[1]);

    let line1 = input[2].split_once(":").unwrap().1;
    let values = line1.split_once(",").unwrap();
    let x0 = values.0.split_once("=").unwrap().1.parse::<usize>().unwrap();
    let y0 = values.1.split_once("=").unwrap().1.parse::<usize>().unwrap();

    let matrix = nalgebra::Matrix2::new(x1 as f64, x2 as f64, y1 as f64, y2 as f64);
    let b = nalgebra::Vector2::new(x0 as f64, y0 as f64);
    let b2 = nalgebra::Vector2::new(x0 as f64 + 1e13, y0 as f64 + 1e13);
    let res = matrix.lu().solve(&b).unwrap();
    let res2 = matrix.lu().solve(&b2).unwrap();

    let part1;
    let part2;
    if (res.x - res.x.round()).abs() < 1e-6 && (res.y - res.y.round()).abs() < 1e-6 {
        part1 = 3*(res.x.round() as u64) + res.y.round() as u64
    }
    else {
        part1 = 0
    }
    if (res2.x - res2.x.round()).abs() < 1e-3 && (res2.y - res2.y.round()).abs() < 1e-3 {
        part2 = 3*(res2.x.round() as u64) + res2.y.round() as u64
    }
    else {
        part2 = 0
    }
    (part1, part2)
}
