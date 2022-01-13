use std::fs;
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let input_str = fs::read_to_string("./inputs/day1.txt")?;
    let input: Vec<i32> = input_str.lines().filter_map(|x| x.parse::<i32>().ok() ).collect();


    println!("Part 1: {}", part1(input.as_slice()));
    println!("Part 2: {}", part2(input.as_slice()));
    Ok(())
}

fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|x| x.0 < x.1)
        .count()
}

fn part2(input: &[i32]) -> usize {
    let data: Vec<i32> = input.windows(3)
        .map(|x| x.iter().sum::<i32>())
        .collect();

    part1(data.as_slice())
}