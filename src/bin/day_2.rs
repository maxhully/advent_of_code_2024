fn is_safe_report(levels: &Vec<i32>) -> bool {
    let initial_ordering = levels[0].cmp(&levels[1]);
    // Must be all either increasing or decreasing
    if initial_ordering.is_eq() {
        return false;
    }
    let pairs = levels.iter().zip(levels.iter().skip(1));
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

fn is_safe_report_with_problem_dampener(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut levels_without_one = levels.clone();
        levels_without_one.remove(i);
        if is_safe_report(&levels_without_one) {
            return true;
        }
    }
    return false;
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

    let num_safe_reports: i32 = reports
        .iter()
        .map(|report| is_safe_report(report) as i32)
        .sum();
    println!("num_safe_reports = {}", num_safe_reports);

    // Puzzle 2
    println!("Engage Problem Dampener!!");
    let num_safe_reports: i32 = reports
        .iter()
        .map(|report| is_safe_report_with_problem_dampener(report) as i32)
        .sum();
    println!("num_safe_reports = {}", num_safe_reports);
}

fn main() {
    day_2();
}
