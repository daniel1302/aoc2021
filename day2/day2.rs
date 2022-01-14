use std::fs;
use std::vec::Vec;

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

fn main() -> std::io::Result<()> {
    let input_str = fs::read_to_string("../inputs/day2.txt")?;

    let input: Vec<Op> = input_str
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .filter(|op: &Vec<&str>| op.len() == 2)
        .map(|op: Vec<&str>| Op::new(op[0], op[1]))
        .collect();
        

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    
    Ok(())
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