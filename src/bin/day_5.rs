use std::{cmp::Ordering, collections::HashSet};

struct PuzzleInputs {
    ordering_rules: HashSet<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse_puzzle_inputs(input_text: &str) -> PuzzleInputs {
    let mut ordering_rules = HashSet::<(i32, i32)>::new();
    let (ordering_rules_text, updates_text) = input_text.split_once("\n\n").unwrap();
    for line in ordering_rules_text.lines() {
        let (left, right) = line.split_once('|').unwrap();
        ordering_rules.insert((left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()));
    }

    let mut updates = Vec::<Vec<i32>>::new();
    for line in updates_text.lines() {
        let update = line
            .split(",")
            .map(|page_number| page_number.parse::<i32>().unwrap())
            .collect();
        updates.push(update);
    }

    PuzzleInputs {
        ordering_rules,
        updates,
    }
}

fn is_update_correctly_ordered(update: &[i32], ordering_rules: &HashSet<(i32, i32)>) -> bool {
    for (i, &page_num) in update.iter().enumerate() {
        for &later_page_num in update[(i + 1)..].iter() {
            // If later_page_num is supposed to come before page_num, this is invalid
            if ordering_rules.contains(&(later_page_num, page_num)) {
                return false;
            }
        }
    }
    true
}

fn get_middle_page_number(update: &[i32]) -> i32 {
    let index = ((update.len() - 1) / 2) as usize;
    update[index]
}

fn day_5() {
    let input_text = std::fs::read_to_string("inputs/day_5.txt").unwrap();
    let inputs = parse_puzzle_inputs(&input_text);

    println!("Puzzle 1!");
    let sum_of_middle_page_numbers: i32 = inputs
        .updates
        .iter()
        .filter(|&update| is_update_correctly_ordered(update, &inputs.ordering_rules))
        .map(|update| get_middle_page_number(update))
        .sum();
    println!("sum_of_middle_page_numbers: {}", sum_of_middle_page_numbers);

    println!("Puzzle 2!");
    let mut incorrectly_ordered_updates: Vec<Vec<i32>> = inputs
        .updates
        .iter()
        .filter(|&update| !is_update_correctly_ordered(update, &inputs.ordering_rules))
        .map(|update| update.clone())
        .collect();
    for update in incorrectly_ordered_updates.iter_mut() {
        update.sort_unstable_by(|&page1, &page2| {
            if inputs.ordering_rules.contains(&(page1, page2)) {
                Ordering::Less
            } else if inputs.ordering_rules.contains(&(page2, page1)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }
    let answer: i32 = incorrectly_ordered_updates
        .iter()
        .map(|update| get_middle_page_number(update))
        .sum();
    println!(
        "sum of the incorrectly ordered, but then corrected updates: {}",
        answer
    );
}

fn main() {
    day_5();
}
