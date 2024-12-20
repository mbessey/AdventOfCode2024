use std::collections::HashSet;


fn main() {
    sample();
    part1();
    part2();
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize
}

impl Direction {
    fn turn_right(& mut self) {
        match *self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }

    fn row_col_dir(&self) -> (isize, isize) {
        match *self {
            Direction::Up    => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down  => (1, 0),
            Direction::Left  => (0, -1),
        }
    }
}

fn sample() {
    let mut values = file_as_vec2("src/sample.txt");
    println!("Rows: {}\tColumns: {}", values.len(), values[0].len());
    pretty_print(&values);
    let start = find_start(&values);
    println!("Start: ({}, {})", start.row, start.col);
    let mut facing = Direction::Up;
    let (visited, steps) = navigate_out(&mut values);
    pretty_print(&values);
    println!("Unique locations visited: {} steps: {}", visited.len(), steps);
}

fn part1() {
    println!("PART ONE:");
    let mut values = file_as_vec2("src/input.txt");
    println!("Rows: {}\tColumns: {}", values.len(), values[0].len());
    // pretty_print(&values);
    let start = find_start(&values);
    println!("Start: ({}, {})", start.row, start.col);
    let (visited, steps) = navigate_out(&mut values);
    println!("Unique locations visited: {} steps: {}", visited.len(), steps);
}

fn part2() {
    println!("PART TWO:");
    let map = file_as_vec2("src/input.txt");
    println!("Rows: {}\tColumns: {}", map.len(), map[0].len());
    // pretty_print(&values);
    let start = find_start(&map);
    println!("Start: ({}, {})", start.row, start.col);
    let mut scratch_map = map.clone();
    let (visited, steps) = navigate_out(&mut scratch_map);
    let step_count = steps;
    println!("Steps: {}", step_count);
    let mut places = 0;
    for coord in visited {
        if coord == start {
            continue;
        }
        let mut new_map = map.clone();
        let obstacle_row = coord.row;
        let obstacle_col = coord.col;
        new_map[obstacle_row][obstacle_col] = 'O';
        // pretty_print(&new_map);
        let (_visited, steps) = navigate_out(&mut new_map);
        if steps > step_count * 2 {
            println!("Count: {}", steps);
            places += 1;
        }
    }
    println!("Places: {}", places)
}

fn navigate_out(v: &mut Vec<Vec<char>>) -> (Vec<Coord>, usize) {
    let mut steps = 0;
    let mut visited = Vec::new();
    let mut row:usize;
    let mut col:usize;
    let max_row = v.len()-1;
    let max_col = v.len()-1;
    let start = find_start(v);
    row = start.row;
    col = start.col;
    let mut facing = Direction::Up;
    let (mut row_dir, mut col_dir) = facing.row_col_dir();
    loop {
        steps += 1;
        // footprint
        v[row][col] = 'X';
        if col == 0 && col_dir == -1 {
            break;
        }
        if col == max_col && col_dir == 1 {
            break;
        }
        if row == 0 && row_dir == -1 {
            break;
        }
        if row == max_row && row_dir == 1 {
            break;
        }
        let new_row = row.checked_add_signed(row_dir).unwrap();
        let new_col = col.checked_add_signed(col_dir).unwrap();
        let ahead = v[new_row][new_col];
        match ahead {
            '.' => {
                row = new_row;
                col = new_col;
                visited.push(Coord { row: row, col: col });
            },
            'X' => {
                row = new_row;
                col = new_col;
            }
            '#' | 'O' => {
                facing.turn_right();
                (row_dir, col_dir) = facing.row_col_dir();
            }
            _ => () // do nothing
        }
        if steps > max_col * max_row * 5 {
            println!("Loop!");
            break;
        }
    }
    return (visited, steps);
}

fn find_start(v: &Vec<Vec<char>>) -> Coord {
    for row in 0..v.len() {
        for col in 0..v[row].len() {
            if v[row][col] == '^' {
                return Coord {row: row, col: col}
            }
        }
    }
    return Coord{ row: 999, col: 999}
}

fn file_as_vec2(path: &str) -> Vec<Vec<char>> {
    let mut result:Vec<Vec<char>> = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        result.push(row.trim().chars().collect());
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

fn pretty_print(vec:&Vec<Vec<char>>) {
    let numbers = 0..vec[0].len();
    let header:Vec<String> = numbers.map(|n|format!("{}", n)).collect();
    println!("\t{:?}", &header);
    for row in 0..vec.len() {
        println!("{}\t{:?}, ", row, &vec[row]);
    }
}
