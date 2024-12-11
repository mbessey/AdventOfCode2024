use std::{collections::HashMap, str::FromStr};

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let mut stones = file_as_vec("src/sample.txt");
    println!("{:?}", stones);
    for _i in 0 ..6 {
        stones = evolve_stones(stones);
        println!("{:?}", stones);
    }
    let mut stones = file_as_vec("src/sample.txt");
    for i in 0..25 {
        stones = evolve_stones(stones);
    }
    println!("After 25 blinks, {} stones", stones.len());
}

fn part1() {
    println!("PART ONE: ");
    let mut stones = file_as_vec("src/part1.txt");
    println!("{:?}", stones);
    for i in 0..25 {
        stones = evolve_stones(stones);
    }
    println!("After 25 blinks, {} stones", stones.len());
}

fn part2() {
    println!("PART TWO: ");
    let mut stones = file_as_vec("src/part1.txt");
    println!("{:?}", stones);
    for i in 0..75 {
        println!("{}: {}", i, stones.len());
        stones = evolve_stones(stones);
    }
    println!("After 75 blinks, {} stones", stones.len());
}

fn evolve_stones(stones: Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    for stone in stones {
        let digits = format!("{}", stone);
        let digits_len = digits.len();
        if stone == 0 {
            result.push(1);
        } else if digits_len % 2 == 0 {
            let top = digits[0..digits_len/2].parse::<i64>().unwrap();
            let bottom = digits[digits_len/2..].parse::<i64>().unwrap();
            result.push(top);
            result.push(bottom);
        } else {
            result.push(stone * 2024);
        }
    }
    return result;
}

fn file_as_vec(path: &str) -> Vec<i64> {
    let mut result = Vec::new();
    let contents = file_contents(path);
    let strings = contents.trim().split_whitespace();
    for s in strings {
        result.push(s.parse::<i64>().expect("parse error"));
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
