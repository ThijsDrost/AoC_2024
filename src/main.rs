use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn time_func(function: fn(&str, bool) -> (i32, i32), base_path: &str, day: u8, real: bool) {
    let start = Instant::now();
    let result = function(base_path, real);
    let duration = start.elapsed();
    println!("Day {:?} part1: {:?}, part 2: {:?}, it took {:?}", day, result.0, result.1, duration);
}


fn main() {
    let base_path = r"C:\Users\drost\RustroverProjects\AoC";
    let start = Instant::now();
    time_func(day1::day1, base_path, 1, true);
    time_func(day2::day2, base_path, 2, true);
    time_func(day3::day3, base_path, 3, true);
    time_func(day4::day4, base_path, 4, true);
    time_func(day5::day5, base_path, 5, true);
    time_func(day6::day6, base_path, 6, true);
    println!("Total time: {:?}", start.elapsed());
}
