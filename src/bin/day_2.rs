fn is_safe_level(level: &Vec<i32>) -> bool {
    let initial_ordering = level[0].cmp(&level[1]);
    // Must be all either increasing or decreasing
    if initial_ordering.is_eq() {
        return false;
    }
    let pairs = level.iter().zip(level.iter().skip(1));
    for (prev, next) in pairs {
        let ordering = prev.cmp(next);
        // The levels are either all increasing or all decreasing.
        if ordering != initial_ordering {
            return false;
        }
        let gap = prev.abs_diff(*next);
        // Any two adjacent levels differ by at least one and at most three.
        // (We already covered the "at least one" part with our ordering handling above)
        if gap > 3 {
            return false;
        }
    }
    true
}

fn day_2() {
    // Puzzle 1
    let contents = std::fs::read_to_string("inputs/day_2.txt").unwrap();
    let levels: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let num_safe_levels: i32 = levels.iter().map(|level| is_safe_level(level) as i32).sum();
    println!("num_safe_levels = {}", num_safe_levels);

    // Puzzle 2
}

fn main() {
    day_2();
}
