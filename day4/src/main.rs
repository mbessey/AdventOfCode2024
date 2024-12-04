
fn main() {
    let values: Vec<Vec<char>> = file_as_vec2("src/sample.txt");
    pretty_print(&values);
    println!("{}", find_xmas(&values));
    part1();
    part2();
}

fn find_xmas(v: &Vec<Vec<char>>) -> i32 {
    let height = v.len();
    let width = v[0].len();
    println!("Width: {}, height: {}", width, height);
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            let q = v[y][x];
            if v[y][x] == 'X' {
                total += search(v, x, y, width, height);
            }
        }
    }
    return total;
}

fn search(v: &Vec<Vec<char>>, x:usize, y:usize, width: usize, height: usize) -> i32 {
    let mut count = 0;
    let expected = ['X', 'M', 'A', 'S'];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if (dx == -1) && (x < 3) {
                continue;
            }
            if (dx == 1) && (x > (width-4)) {
                continue;
            }
            if (dy == -1) && (y < 3) {
                continue;
            }
            if (dy == 1) && (y > (height-4)) {
                continue;
            }
            let mut matched = 0;
            let mut sx = x;
            let mut sy = y;
            for i in 1..4 {
                sx = sx.checked_add_signed(dx)
                .expect("should not overflow");
                sy = sy.checked_add_signed(dy)
                .expect("should not overflow");
                let c = v[sy][sx];
                if c != expected[i] {
                    break;
                } else {
                    matched += 1;
                }
            }
            if matched == 3 {
                count += 1;
            }
        }
    }
    return count;
}

fn part1() {
    println!("==========\nPART ONE\n==========");
    let values: Vec<Vec<char>> = file_as_vec2("src/part1.txt");
    println!("{}", find_xmas(&values));
}

fn part2() {

}

fn file_as_vec2(path: &str) -> Vec<Vec<char>> {
    let mut result:Vec<Vec<char>> = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        let mut row_vec:Vec<char> = Vec::new();
        let values = row.chars();
        for value in values {
            row_vec.push(value);
        }
        result.push(row_vec);
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

fn pretty_print(vec:&Vec<Vec<char>>) {
    for row in vec {
        println!("{:?}, ", row);
    }
}
