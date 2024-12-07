
fn main() {
    // sample();
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
    let (row, col) = find_start(&values);
    println!("Start: ({}, {})", row, col);
    let mut facing = Direction::Up;
    let visited = navigate_out(&mut values);
    pretty_print(&values);
    println!("Unique locations visited: {}", &visited);
}

fn part1() {
    let mut values = file_as_vec2("src/part1.txt");
    println!("Rows: {}\tColumns: {}", values.len(), values[0].len());
    // pretty_print(&values);
    let (row, col) = find_start(&values);
    println!("Start: ({}, {})", row, col);
    let mut facing = Direction::Up;
    let visited = navigate_out(&mut values);
    println!("Unique locations visited: {}", &visited);
    // pretty_print(&values);
}

fn part2() {

}

fn navigate_out(v: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 1;
    let mut row:usize;
    let mut col:usize;
    let max_row = v.len()-1;
    let max_col = v.len()-1;
    (row, col) = find_start(v);
    let mut facing = Direction::Up;
    let (mut row_dir, mut col_dir) = facing.row_col_dir();
    loop {
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
                count += 1;
            },
            'X' => {
                row = new_row;
                col = new_col;
            }
            '#' => {
                facing.turn_right();
                (row_dir, col_dir) = facing.row_col_dir();
            }
            _ => () // do nothing
        }
    }
    return count;
}

fn find_start(v: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..v.len() {
        for col in 0..v[row].len() {
            if v[row][col] == '^' {
                return (row,col)
            }
        }
    }
    return (999,999)
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
