use std::{collections::HashMap, fmt, };


fn main() {
    sample();
    part1();
    part2();
}

struct Data {
    before:HashMap<i32, Vec<i32>>,
    updates:Vec<Vec<i32>>,
}

impl Data {
    fn new(s: &String) -> Data {
        let mut d = Data {
            before: HashMap::new(),
            updates: Vec::new()
        };
        for line in s.lines() {
            if line.contains("|") {
                // before
                let parts:Vec<&str> = line.split("|").collect();
                let key = parts[0].parse::<i32>()
                    .expect("didn't parse");
                let val = parts[1].parse::<i32>()
                    .expect("didn't parse");
                let v = d.before.get_mut(&key);
                match v {
                    Some(i) => i.push(val),
                    None => {
                        let mut v:Vec<i32> = Vec::new();
                        v.push(val);
                        d.before.insert(key, v);
                    }
                }
            } else if line.contains(",") {
                // update
                let parts = line.split(",");
                let pages:Vec<i32> = parts.map(|n| n.parse::<i32>().expect("parse error")).collect();
                d.updates.push(pages);
            } else if line.trim().len() == 0 {
                // blank
            } else {
                panic!("Why?")
            }
        }
        return d;
    }

    fn check_updates(&self) {
        for update in &self.updates {
            if self.check_update(update) {
                print!("pass ");
            } else {
                print!("FAIL ");
            }
            println!("{:?}", update);
        }
    }
    fn check_update(&self, update:&Vec<i32>) -> bool {
        for i in 1..update.len() {
            let page = update[i];
            let before = self.before.get(&page);
            match before {
                None => continue,
                Some(v) => {
                    // Big-O from heck...
                    for needle in v {
                        for j in 0..i {
                            if update[j] == *needle {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        return true;
    }
}

impl fmt::Display for Data {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for (key, value) in &self.before {
            writeln!(f, "{} | {:?}", key, value)
                .expect("couldn't write");
        }
        writeln!(f)
            .expect("error writing");
        for page in &self.updates {
            writeln!(f, "{:?},", page)
                .expect("couldn't write");
        }
        return Ok(());
    }
}

fn sample() {
    println!("SAMPLE DATA");
    let s = file_contents("src/sample.txt");
    let d = Data::new(&s);
    // println!("{}", d);
    d.check_updates();
}

fn part1() {
    println!("PART ONE");
}

fn part2() {
    println!("PART TWO");
}

fn file_as_vec2(path: &str) -> Vec<Vec<i32>> {
    let mut result:Vec<Vec<i32>> = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        let mut row_vec:Vec<i32> = Vec::new();
        let values = row.split_whitespace();
        result.push(row_vec);
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

fn pretty_print(vec:&Vec<Vec<i32>>) {
    for row in vec {
        println!("{:?}, ", row);
    }
}
