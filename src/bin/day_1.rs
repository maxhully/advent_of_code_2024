use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;

fn parse_puzzle_inputs(
    lines: impl Iterator<Item = String>,
) -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    for line in lines {
        let split_line = line.split_whitespace();
        let parts: Vec<&str> = split_line.collect();
        // Ignore lines that don't have two things on them, but log them
        if parts.len() != 2 {
            println!("Less than two parts on this line: {:?}", line);
        }
        left.push(i32::from_str_radix(parts[0], 10)?);
        right.push(i32::from_str_radix(parts[1], 10)?);
    }
    Ok((left, right))
}

fn day_1() {
    // Puzzle 1
    let (mut left, mut right) = {
        let file = File::open("inputs/day_1.txt").unwrap();
        parse_puzzle_inputs(io::BufReader::new(file).lines().flatten()).unwrap()
    };
    left.sort();
    right.sort();
    let total_distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(left_num, right_num)| left_num.abs_diff(*right_num))
        .sum();
    println!("total_distance = {}", total_distance);

    // Puzzle 2
    let mut right_cursor: usize = 0;
    let similarity_score: i32 = left
        .iter()
        .map(|left_num| {
            let mut num_appearances_in_right = 0;
            // Taking advantage of the fact that we sorted these Vecs above
            while right[right_cursor] < *left_num {
                right_cursor += 1;
            }
            while right[right_cursor] == *left_num {
                num_appearances_in_right += 1;
                right_cursor += 1;
            }
            left_num * num_appearances_in_right
        })
        .sum();
    println!("similarity_score = {}", similarity_score);
}

fn main() {
    day_1();
}
