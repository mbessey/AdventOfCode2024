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
        let mut total = 0;
        for update in &self.updates {
            if self.check_update(update) {
                let mid = self.middle(update);
                println!("pass {:?}", update);
                println!("middle: {}", mid);
                total += mid;
            } else {
                println!("FAIL  {:?}", update)
            }
        }
        println!("TOTAL: {}", total);
    }

    fn incorrect_updates(&self) -> Vec<Vec<i32>> {
        let mut bads: Vec<Vec<i32>> = Vec::new();
        for update in &self.updates {
            if !self.check_update(update) {
                bads.push(update.to_vec());
            }
        }
        return bads;
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

    fn middle(&self, update:&Vec<i32>) -> i32 {
        if update.len() % 2 ==0 {
            panic!("What's the middle value of an even interval?");
        }
        let mid_index = update.len() / 2;
        return update[mid_index];
    }

    fn fix(&self, update:&Vec<i32>) -> Vec<i32> {
        let v:Vec<i32> = Vec::new();
        for page in update {

        }
        return v;
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
    let s = file_contents("src/part1.txt");
    let d = Data::new(&s);
    d.check_updates();
}

fn part2() {
    println!("PART TWO");
    let s = file_contents("src/sample.txt");
    let d = Data::new(&s);
    let bads = d.incorrect_updates();
    for bad in bads {
        let fixed = d.fix(bad);
        println!("Fixed: {:?}", fixed);
    }
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
