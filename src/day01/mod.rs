use std::fs;
use std::vec::Vec;
use std::result::Result;
use std::error::Error;


fn part1(input: &[i32]) -> i32 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|x| x.0 < x.1)
        .count() as i32
}

fn part2(input: &[i32]) -> i32 {
    let data: Vec<i32> = input.windows(3)
        .map(|x| x.iter().sum::<i32>())
        .collect();

    part1(data.as_slice())
}


pub fn run(file: &str) -> Result<(i32, i32), Box<dyn Error>>{
    let input_str = fs::read_to_string(file)?;
    let input: Vec<i32> = input_str.lines().filter_map(|x| x.parse::<i32>().ok() ).collect();

    Ok((part1(input.as_slice()), part2(input.as_slice())))
}

#[test]
fn test_run() {
    assert_eq!(run("./inputs/day1.txt").unwrap(), (1266, 1217))
}