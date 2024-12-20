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
    let rows = contents.split("\n");
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();
    for row in rows {
        // print!("{row}\n");
        let mut words = row.split_whitespace();
        let l = words.next().unwrap();
        let r = words.next().unwrap();
        let li = l.parse::<i32>()
            .expect("couldn't parser");
        let ri = r.parse::<i32>()
            .expect("couldn't parser");
        left.push(li);
        right.push(ri);
    }
    left.sort();
    right.sort();
    let mut dist = 0;
    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];
        let d:i32;
        if l > r {
            d = l - r;
        } else {
            d = r - l;
        }
        dist += d;
        // print!("{l}\t{r}\n");
    }
    print!("Distance: {dist}\n")
}

fn part2() {
    let contents = fs::read_to_string("src/part1.txt")
        .expect("should have read the file");
    let rows = contents.split("\n");
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();
    for row in rows {
        // print!("{row}\n");
        let mut words = row.split_whitespace();
        let l = words.next().unwrap();
        let r = words.next().unwrap();
        let li = l.parse::<i32>()
            .expect("couldn't parser");
        let ri = r.parse::<i32>()
            .expect("couldn't parser");
        left.push(li);
        right.push(ri);
    }
    // Don't need to sort for this version
    // left.sort();
    // right.sort();

    assert_eq!(left.len(), right.len());
    let mut similarity = 0;
    for i in 0..left.len() {
        let l = left[i];
        let mut count = 0;
        for j in 0..right.len() {
            let r = right[j];
            if r == l {
                count += 1;
            }
        }
        similarity += l * count;
    }
    print!("Similarity: {similarity}\n")
}
