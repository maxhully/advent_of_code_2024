fn is_safe_report_1d(
    levels: &[i32],
    bad_level_tolerance: usize,
    min_gap_size: u32,
    max_gap_size: u32,
    start: i32,
    end: i32,
    step: i32,
) -> bool {
    assert!(min_gap_size < max_gap_size);
    assert!(start >= 0 && end >= 0);
    let mut i = start;
    let mut j = start + step;
    let mut num_good_gaps = 0;
    let mut remaining_bad_level_budget = bad_level_tolerance;
    while j != end {
        let gap = levels[j as usize].abs_diff(levels[i as usize]);
        // println!("i = {}, j = {}", i, j);
        // println!("gap = {}", gap);
        let is_good_gap = min_gap_size <= gap && gap <= max_gap_size;
        if is_good_gap {
            num_good_gaps += 1;
            i = j;
            j += step;
        } else if remaining_bad_level_budget == 0 {
            // println!("too many bad levels! not safe!! (i = {}, j = {})", i, j);
            break;
        } else {
            remaining_bad_level_budget -= 1;
            // println!("bad level, skipping! (i = {}, j = {})", i, j);
            // Try skipping this one (on the next iteration)
            j += step;
        }
    }
    let target_good_gaps = levels.len() - bad_level_tolerance - 1;
    // if num_good_gaps == target_good_gaps - 1 {
    //     println!("levels = {:?}", levels);
    //     println!("num_good_gaps = {}\n", num_good_gaps);
    // }
    num_good_gaps >= target_good_gaps
}

fn is_safe_report(levels: &Vec<i32>, bad_level_tolerance: usize) -> bool {
    assert!(bad_level_tolerance == 0 || bad_level_tolerance == 1);

    // Fuck. Now this is over-shooting it. I think it might have to do with the edge
    // conditions.

    // Check if it's a good report in the increasing direction
    // println!("forward");
    is_safe_report_1d(
        &levels,
        bad_level_tolerance,
        1,
        3,
        0,
        (levels.len() - 1) as i32,
        1,
    ) || is_safe_report_1d(
        &levels,
        bad_level_tolerance,
        1,
        3,
        (levels.len() - 1) as i32,
        0,
        -1,
    )
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
        .map(|report| is_safe_report(report, 0) as i32)
        .sum();
    println!("num_safe_reports = {}", num_safe_reports);

    // Puzzle 2
    println!("Engaging Problem Dampener!!");
    let num_safe_reports: i32 = reports
        .iter()
        .map(|report| is_safe_report(report, 1) as i32)
        .sum();
    println!("num_safe_reports = {}", num_safe_reports);

    let report = vec![74, 77, 79, 82, 81, 82];
    println!("levels = {:?}", report);
    let is_safe = is_safe_report(&report, 1);
    println!("is_safe = {}", is_safe);
}

fn main() {
    day_2();
}
