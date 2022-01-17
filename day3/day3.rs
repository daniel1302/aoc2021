use std::fs;
use std::vec::Vec;

#[inline(always)]
fn bit_on_pos(num: &u32, pos: u32) -> u8 {
    ((num >> pos) & 0b1) as u8
} 

fn main() -> std::io::Result<()> {
    let input_str = fs::read_to_string("../inputs/day3.txt")?;
    let input: Vec<u32> = input_str
        .lines()
        .map(|num| isize::from_str_radix(num, 2).unwrap() as u32 )
        .collect();


    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn most_common_bits(word_set: &[u32]) -> u32 {
    const U32_SIZE: usize = u32::BITS as usize;

    let mut result: u32 = 0;
    let mut zeros = vec![0; U32_SIZE];
    let mut ones = vec![0; U32_SIZE];

    for word in word_set {
        for bit_pos in 0..u32::BITS-1 {
            match bit_on_pos(word, bit_pos) {
                0 => zeros[bit_pos as usize] += 1,
                _ => ones[bit_pos as usize] += 1,
            }
        }
    }

    for bit_pos in 0..u32::BITS-1 {
        if zeros[bit_pos as usize] < ones[bit_pos as usize] {
            result += 1 << bit_pos as usize;
        }
    }

    result
}

fn part1(input: &[u32]) -> u32 {
    let gamma = most_common_bits(input);
    let epsilon = !gamma & u32::MAX >> (gamma.leading_zeros()-1);
    
    gamma * epsilon
}