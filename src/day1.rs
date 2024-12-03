use std::fs;

pub fn day1() {
    let values = fs::read_to_string(r"C:\Users\drost\RustroverProjects\AoC\inputs\day1_sample.txt").unwrap();
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = values.lines()
        .filter_map(|line| line.split_once("   "))
        .filter_map(|(first, second)| {
            let first = first.parse::<i32>().ok();
            let second = second.parse::<i32>().ok();
            first.zip(second)
        })
        .unzip();

    vec1.sort_unstable();
    vec2.sort_unstable();

    let sum: i32 = vec1.iter()
        .zip(vec2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    println!("Day 1, part 1: {}", sum);

    let mut index1 = 0;
    let mut index2 = 0;
    let mut total = 0;
    let mut num1 = 0;
    let mut num2 = 1;
    while index1 < vec1.len() && index2 <vec2.len() {
        if vec1[index1] < vec2[index2] {
            index1 += 1;
            continue;
        }
        else if vec1[index1] > vec2[index2] {
            index2 += 1;
            continue;
        }
        while vec1[index1] == vec2[index2] {
            num1 += 1;
            index2 += 1;
        }
        while (index1 + num2 < vec1.len()) && (vec1[index1] == vec1[index1 + num2]) {
            num2 += 1;
        }
        total += num1*num2*(vec1[index1] as usize);
        index1 += num2;
        num1 = 0;
        num2 = 1;
    }
    println!("Day 1, part 2: {}", total);
}