use std::{collections::HashMap};

#[derive(Debug)]
struct Coord {
    row: i32,
    col: i32
}

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let values = file_as_vec2("src/sample.txt");
    //pretty_print(&values);
}

fn part1() {
    println!("PART ONE: ");

}

fn part2() {
    println!("PART TWO: ");

}

fn file_as_vec2(path: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
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
