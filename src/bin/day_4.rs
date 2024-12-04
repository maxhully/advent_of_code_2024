const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
];

fn has_xmas_at_position_in_direction(
    grid: &Vec<Vec<char>>,
    coordinates: (i32, i32),
    direction: (i32, i32),
) -> bool {
    assert!(-1 <= direction.0 && direction.0 <= 1);
    assert!(-1 <= direction.1 && direction.1 <= 1);
    let (start_x, start_y) = coordinates;
    let (dx, dy) = direction;
    for (i, expected_letter) in XMAS.iter().enumerate() {
        let i = i as i32;
        let x = start_x + dx * i;
        let y = start_y + dy * i;
        let Some(line) = grid.get(y as usize) else {
            return false;
        };
        let Some(letter) = line.get(x as usize) else {
            return false;
        };
        if letter != expected_letter {
            return false;
        }
    }
    true
}

fn get_from_grid(grid: &Vec<Vec<char>>, coordinates: (i32, i32)) -> char {
    let (x, y) = coordinates;
    grid[y as usize][x as usize]
}

fn has_x_mas_at_position(grid: &Vec<Vec<char>>, coordinates: (i32, i32)) -> bool {
    let (x, y) = coordinates;
    if grid[y as usize][x as usize] != 'A' {
        return false;
    }
    let up_left = (x - 1, y - 1);
    let bottom_right = (x + 1, y + 1);
    let up_right = (x + 1, y - 1);
    let bottom_left = (x - 1, y + 1);
    // Jeez
    ((get_from_grid(grid, up_left) == 'M' && get_from_grid(grid, bottom_right) == 'S')
        || (get_from_grid(grid, up_left) == 'S' && get_from_grid(grid, bottom_right) == 'M'))
        && ((get_from_grid(grid, up_right) == 'M' && get_from_grid(grid, bottom_left) == 'S')
            || (get_from_grid(grid, up_right) == 'S' && get_from_grid(grid, bottom_left) == 'M'))
}

fn count_all_xmases(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let mut xmas_positions_and_directions = Vec::<((usize, usize), (i32, i32))>::new();
    for (y, chars) in grid.iter().enumerate() {
        for x in 0..chars.len() {
            for dir in DIRECTIONS.iter() {
                if has_xmas_at_position_in_direction(grid, (x as i32, y as i32), *dir) {
                    count += 1;
                    xmas_positions_and_directions.push(((x, y), *dir));
                }
            }
        }
    }
    // I'm also printing out the xmas grid, to check my work (and for fun.)
    // This could be split up into separate functions, of course.
    let mut blank_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|line| line.iter().map(|_letter| '.').collect())
        .collect();
    for ((start_x, start_y), (dx, dy)) in xmas_positions_and_directions.iter() {
        for (i, letter) in XMAS.iter().enumerate() {
            let i = i as i32;
            let x = (*start_x as i32) + dx * i;
            let y = (*start_y as i32) + dy * i;
            blank_grid[y as usize][x as usize] = *letter;
        }
    }
    for letters in blank_grid.iter() {
        let line: String = letters.into_iter().collect();
        println!("{}", line);
    }

    count
}

fn count_all_x_mases(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for (y, chars) in grid.iter().enumerate() {
        for x in 0..chars.len() {
            // Any cells on the edge can't be it
            if x == 0 || y == 0 || x == chars.len() - 1 || y == grid.len() - 1 {
                continue;
            }
            if has_x_mas_at_position(grid, (x as i32, y as i32)) {
                count += 1;
            }
        }
    }
    count
}

fn day_4() {
    println!("Puzzle 1!");
    let word_search_text = std::fs::read_to_string("inputs/day_4.txt").unwrap();
    let word_search_grid: Vec<Vec<char>> = word_search_text
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let count = count_all_xmases(&word_search_grid);
    println!("count: {}", count);

    println!("Puzzle 2!");
    let count = count_all_x_mases(&word_search_grid);
    println!("count: {}", count);
}

fn main() {
    day_4();
}
