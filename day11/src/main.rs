use std::collections::HashMap;

fn main() {
    sample();
    // part1();
    // part2();
}

type Stones = HashMap<i64, i64>;

fn sample() {
    println!("Sample: ");
    let mut stones = file_as_stones("src/sample.txt");
    println!("{:?}", stones);
    for _i in 0 ..6 {
        stones = evolve_stones(stones);
        println!("{:?}", stones);
    }
    let total = totalize(&stones);
    println!("After 6 blinks, {} stones", total);
    let mut stones = file_as_stones("src/sample.txt");
    for _i in 0..25 {
        stones = evolve_stones(stones);
    }
    let total = totalize(&stones);
    println!("After 25 blinks, {} stones", total);
}

fn part1() {
    println!("PART ONE: ");
    let mut stones = file_as_stones("src/part1.txt");
    println!("{:?}", stones);
    for i in 0..25 {
        stones = evolve_stones(stones);
    }
    println!("After 25 blinks, {} stones", stones.len());
}

fn part2() {
    println!("PART TWO: ");
    let mut stones = file_as_stones("src/sample.txt");
    println!("{:?}", stones);
    for stone in stones.keys() {
        let babies = blink(*stone, 75);
    }
    println!("After 75 blinks, {} stones", stones.len());
}

fn blink(stone: i64, num_blinks: i64) -> Stones {
    let mut stone = stone;
    let mut result = Stones::new();
    for _blink in 0..num_blinks {
        let digits = format!("{}", stone);
        let digits_len = digits.len();
        if stone == 0 {
            stone = 1;
        } else if digits_len % 2 == 0 {
            stone = digits[0..digits_len/2].parse::<i64>().unwrap();
            let bottom = digits[digits_len/2..].parse::<i64>().unwrap();
            *result.entry(bottom).or_insert(0) += 1;
        } else {
            stone = stone * 2024;
        }
    }
    return result;
}

fn evolve_stones(stones: Stones) -> Stones {
    let mut result = Stones::new();
    for (stone, count) in stones {
        let digits = format!("{}", stone);
        let digits_len = digits.len();
        if stone == 0 {
            *result.entry(1).or_insert(0) += count;
        } else if digits_len % 2 == 0 {
            let top = digits[0..digits_len/2].parse::<i64>().unwrap();
            let bottom = digits[digits_len/2..].parse::<i64>().unwrap();
            *result.entry(top).or_insert(0) += count;
            *result.entry(bottom).or_insert(0) += count;
        } else {
            *result.entry(stone * 2024).or_insert(0) += count;
        }
    }
    return result;
}

fn totalize(stones: &Stones) -> i64 {
    let mut total = 0;
    for (_stone, count) in stones {
        total += count;
    }
    return total;
}

fn file_as_stones(path: &str) -> Stones {
    let mut result = Stones::new();
    let contents = file_contents(path);
    let strings = contents.trim().split_whitespace();
    for s in strings {
        let v =s.parse::<i64>().expect("parse error"); 
        result.insert(v, 1i64);
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
