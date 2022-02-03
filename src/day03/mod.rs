use std::fs;
use std::vec::Vec;
use std::result::Result;
use std::error::Error;

fn bit_on_pos(num: &u32, pos: u32) -> u8 {
    ((num >> pos) & 0b1) as u8
}

fn most_common_bits(word_set: &[u32], soft: bool) -> u32 {
    const U32_SIZE: usize = u32::BITS as usize;

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

    (0..u32::BITS-1)
        .filter(|&pos| (soft && zeros[pos as usize] <= ones[pos as usize]) || (!soft && zeros[pos as usize] < ones[pos as usize]))
        .fold(0, |acc, pos| acc + (1<<pos as usize))
}

fn part1(input: &[u32]) -> u32 {
    let gamma = most_common_bits(input, true);
    let epsilon = !gamma & u32::MAX >> (gamma.leading_zeros()-1);
    
    gamma * epsilon
}

fn part2(input: &[u32]) -> u32 {
    let compute_rating = |input_set: &[u32], word_size: u32, equals_bits: bool| -> u32 {
        let mut words_set = input_set.to_vec();
        let mut msb_in_set: u32;

        for pos in (0..word_size).rev() {
            msb_in_set = most_common_bits(&words_set, true);
            match words_set.len() {
                2.. => words_set = words_set
                    .iter()
                    .filter(|&word| (bit_on_pos(word, pos) == bit_on_pos(&msb_in_set, pos)) == equals_bits )
                    .map(|&w| w)
                    .collect(),
                _ => {},
            }
        }
       
        *words_set.first().unwrap_or(&0)
    };

    let word_size: u32 = u32::BITS - input.iter().fold(u32::BITS, |min, word| std::cmp::min(min, word.leading_zeros()));
    let oxygen_generator_rating = compute_rating(input, word_size, true);
    let co2_scrubber_rating = compute_rating(input, word_size, false);

    oxygen_generator_rating * co2_scrubber_rating
}

fn run(file_path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let input_str = fs::read_to_string(file_path)?;
    let input: Vec<u32> = input_str
        .lines()
        .map(|num| isize::from_str_radix(num, 2).unwrap() as u32 )
        .collect();

    Ok((part1(&input) as i32, part2(&input) as i32))
}

#[test]
fn test_run() {
    assert_eq!(run("./inputs/day3.txt").unwrap(), (3429254, 5410338))
}