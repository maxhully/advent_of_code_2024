use regex::Regex;

enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

fn parse_instructions(program_text: &str) -> Vec<Instruction> {
    let instruction_pattern = Regex::new(
        r"(?<mul>mul)\((?<left>[0-9]+),(?<right>[0-9]+)\)|(?<do>do)\(\)|(?<dont>don't)\(\)",
    )
    .unwrap();

    instruction_pattern
        .captures_iter(&program_text)
        .map(|captures| {
            if captures.name("mul").is_some() {
                let left = captures.name("left").unwrap().as_str();
                let right = captures.name("right").unwrap().as_str();
                Instruction::Mul(left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
            } else if captures.name("do").is_some() {
                Instruction::Do
            } else if captures.name("dont").is_some() {
                Instruction::Dont
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn day_3() {
    // Puzzle 1
    println!("Puzzle 1!");
    let program_text = std::fs::read_to_string("inputs/day_3.txt").unwrap();
    let mul_instruction_pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mul_instructions = mul_instruction_pattern
        .captures_iter(&program_text)
        .map(|captures| {
            let (_instruction, [left, right]) = captures.extract();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        });

    let answer: i32 = mul_instructions.map(|(left, right)| left * right).sum();
    println!("total: {}", answer);

    // Puzzle 2
    println!("Puzzle 2!");
    let instructions = parse_instructions(&program_text);
    let mut total = 0;
    let mut is_enabled = true;
    for instruction in instructions {
        match instruction {
            Instruction::Do => {
                is_enabled = true;
            }
            Instruction::Dont => {
                is_enabled = false;
            }
            Instruction::Mul(left, right) => {
                if is_enabled {
                    total += left * right;
                }
            }
        }
    }
    println!("total: {}", total);
}

fn main() {
    day_3();
}
