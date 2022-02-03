use std::fs;
use std::vec::Vec;
use std::result::Result;
use std::error::Error;

#[derive(Debug)]
enum OpCode {
    None,
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Op {
    code: OpCode,
    operand: i32,
}

impl Op {
    pub fn new(code: &str, operand: &str) -> Self {
        let code = match code {
            "forward" => OpCode::Forward,
            "down" => OpCode::Down,
            "up" => OpCode::Up,
            _ => OpCode::None,
        };

        let operand = operand.parse().unwrap();

        Op{code, operand} 
    }
}

fn part1(input: &[Op]) -> i32 {
    let (mut x, mut y) = (0i32, 0i32);
    input.iter().for_each(|op| match op.code {
        OpCode::Forward => x += op.operand,
        OpCode::Down => y += op.operand,
        OpCode::Up => y -= op.operand,
        _ => {},
    });

    x*y
}


fn part2(input: &Vec<Op>) -> i32 {
    let (mut x, mut y, mut aim) = (0i32, 0i32, 0i32);
    input.iter().for_each(|op| match op.code {
        OpCode::Forward =>  { x += op.operand; y += aim*op.operand },
        OpCode::Down => aim += op.operand,
        OpCode::Up => aim -= op.operand,
        _ => {},
    });

    x*y
}

pub fn run(input_file: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let input_str = fs::read_to_string(input_file)?;

    let input: Vec<Op> = input_str
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .filter(|op: &Vec<&str>| op.len() == 2)
        .map(|op: Vec<&str>| Op::new(op[0], op[1]))
        .collect();

    Ok((part1(&input), part2(&input)))
}

#[test]
fn test_run() {
    assert_eq!(run("./inputs/day2.txt").unwrap(), (1250395, 1451210346))
}