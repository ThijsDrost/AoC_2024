use std::time::Instant;
use std::fmt::Debug;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

pub fn time_func<T: Debug, S: Debug>(function: fn(&str, bool) -> (T, S), base_path: &str, day: u8, real: bool) {
    let start = Instant::now();
    let result = function(base_path, real);
    let duration = start.elapsed();
    println!("Day {:?} part 1: {:?}, part 2: {:?}, it took {:?}", day, result.0, result.1, duration);
}


fn main() {
    let base_path = r"C:\Users\drost\RustroverProjects\AoC_2024";
    let start = Instant::now();
    time_func(day1::day1, base_path, 1, false);
    time_func(day2::day2, base_path, 2, false);
    time_func(day3::day3, base_path, 3, false);
    time_func(day4::day4, base_path, 4, false);
    time_func(day5::day5, base_path, 5, false);
    time_func(day6::day6, base_path, 6, false);
    time_func(day7::day7, base_path, 7, false);
    time_func(day8::day8, base_path, 8, false);
    time_func(day9::day9, base_path, 9, false);
    time_func(day10::day10, base_path, 10, true);
    println!("Total time: {:?}", start.elapsed());
}
