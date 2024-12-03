use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("PART ONE");
    let text = file_contents("src/part1.txt");
    let txt = text.as_str();
    let re= Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum = 0;
    for capture_val in re.captures_iter(txt) {
        print!("{}, {} * {} = ", &capture_val[0], &capture_val[1], &capture_val[2]);
        let l = capture_val[1].parse::<i32>().unwrap();
        let r = capture_val[2].parse::<i32>().unwrap();
        println!("{}", l * r);
        sum += (l * r);
    }
    println!("==========================\n{}", sum);
}

fn part2() {
    println!("PART TWO");
    let text = file_contents("src/part1.txt");
    let txt = text.as_str();
    let re= Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut active = true;
    for capture_val in re.captures_iter(txt) {
        let instruction = &capture_val[0];
        match instruction {
            "do()" => {
                active = true;
                println!("DO");
            },
            "don't()" => {
                active = false;
                println!("DON'T");
            },                
            _ => {
                if active {
                    print!("{}, {} * {} = ", &capture_val[0], &capture_val[1], &capture_val[2]);
                    let l = capture_val[1].parse::<i32>().unwrap();
                    let r = capture_val[2].parse::<i32>().unwrap();
                    println!("{}", l * r);
                    sum += (l * r);
                } else {
                    println!("skipped");
                }
            }
        }
    }
    println!("==========================\n{}", sum);
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
