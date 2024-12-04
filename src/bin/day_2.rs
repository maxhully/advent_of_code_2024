use std::cmp::Ordering;

struct LevelDiff {
    ordering: Ordering,
    gap: u32,
}

impl LevelDiff {
    fn from_pair(prev: i32, next: i32) -> LevelDiff {
        LevelDiff {
            ordering: prev.cmp(&next),
            gap: prev.abs_diff(next),
        }
    }
}

fn is_safe_level(levels: &Vec<i32>, bad_level_tolerance: usize) -> bool {
    println!("\nlevels = {:?}", levels);
    let diffs: Vec<LevelDiff> = levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|(prev, next)| LevelDiff::from_pair(*prev, *next))
        // Filter out the level diffs with too-large gaps right away
        .filter(|diff| 1 <= diff.gap && diff.gap <= 3)
        .collect();

    println!("levels.len() = {}", levels.len());
    // Now, see if we have enough good levels (at least the total - bad_level_tolerance)
    // in either the increasing or decreasing direction.
    let mut increasing_count: usize = 0;
    let mut decreasing_count: usize = 0;
    for gap in diffs.iter() {
        match gap.ordering {
            Ordering::Greater => {
                increasing_count += 1;
            }
            Ordering::Less => {
                decreasing_count += 1;
            }
            Ordering::Equal => (),
        }
    }
    println!("increasing_count = {}", increasing_count);
    println!("decreasing_count = {}", decreasing_count);

    let target_count = levels.len() - 1 - bad_level_tolerance;
    let is_safe = increasing_count >= target_count || decreasing_count >= target_count;
    if !is_safe {
        println!("not safe!!");
    }
    is_safe
}

fn day_2() {
    // Puzzle 1
    let contents = std::fs::read_to_string("inputs/day_2.txt").unwrap();
    let reports: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // let num_safe_reports: i32 = reports
    //     .iter()
    //     .map(|level| is_safe_level(level, 0) as i32)
    //     .sum();
    // println!("num_safe_reports = {}", num_safe_reports);

    // Puzzle 2
    println!("Engaging Problem Dampener!!");
    let num_safe_reports: i32 = reports
        .iter()
        // .take(12)
        .map(|level| is_safe_level(level, 1) as i32)
        .sum();
    println!("num_safe_reports = {}", num_safe_reports);
}

fn main() {
    day_2();
}
