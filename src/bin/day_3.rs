use regex::Regex;

fn day_3() {
    // Puzzle 1
    let program_text = std::fs::read_to_string("inputs/day_3.txt").unwrap();
    let mul_instruction_pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mul_instructions = mul_instruction_pattern
        .captures_iter(&program_text)
        .map(|captures| {
            let (_instruction, [left, right]) = captures.extract();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        });

    let answer: i32 = mul_instructions.map(|(left, right)| left * right).sum();
    println!("{}", answer);

    // Puzzle 2
}

fn main() {
    day_3();
}
