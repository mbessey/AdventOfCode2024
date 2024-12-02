// use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    part1();
    part2();
}

fn part1() {
    // let cwd = env::current_dir()
    //     .expect("should have gotten the CWD");
    let contents = fs::read_to_string("src/part1.txt")
        .expect("should have read the file");
    let mut safe_count = 0;
    let rows = contents.lines();
    'row:for row in rows {
        let mut words = row.split_whitespace();
        let mut last = words.next().unwrap().parse::<i32>().unwrap();
        let mut direction = 0;
        for word in words {
            let current = word.parse::<i32>().unwrap();
            if current > last {
                if direction == 0 {
                    direction = 1;
                }
                if direction == -1 {
                    // direction changed. Not safe
                    continue 'row;
                }
                if (current - last) > 3 {
                    //  Big change. Not saf
                    continue 'row;
                }
            } else if current < last {
                if direction == 0 {
                    direction = -1;
                }
                if direction == 1 {
                    // Direction change. Not safe
                    continue 'row;
                }
                if (last - current) > 3 {
                    // Big change. Not safe
                    continue 'row;
                }
            } else {
                // Equal. not safe
                continue 'row;
            }
            last = current;
        }
        safe_count += 1;
    }
    print!("Safe: {safe_count}\n")
}


fn part2() {
    // let cwd = env::current_dir()
    //     .expect("should have gotten the CWD");
    let contents = fs::read_to_string("src/part1.txt")
        .expect("should have read the file");
    let mut safe_count = 0;
    let rows = contents.lines();
    for row in rows {
        let words:Vec<&str> = row.split_whitespace().collect();
        if check_safe(&words) {
            safe_count += 1;
        } else {
            for i in 0..words.len() {
                let mut new_words = words.clone();
                new_words.remove(i);
                if check_safe(&new_words) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    print!("Safe: {safe_count}\n")
}

fn check_safe(words:&Vec<&str>) -> bool {
    let mut last = words[0].parse::<i32>().unwrap();
    let mut direction = 0;
    for i in 1..words.len()  {
        let word = words[i];
        let current = word.parse::<i32>().unwrap();
        if current > last {
            if direction == 0 {
                direction = 1;
            }
            if direction == -1 {
                // direction changed. Not safe
                return false;
            }
            if (current - last) > 3 {
                //  Big change. Not saf
                return false;
            }
        } else if current < last {
            if direction == 0 {
                direction = -1;
            }
            if direction == 1 {
                // Direction change. Not safe
                return false;
            }
            if (last - current) > 3 {
                // Big change. Not safe
                return false;
            }
        } else {
            // Equal. not safe
            return false;
        }
        last = current;
    }
    return true;
}
