use std::char::ToLowercase;

fn main() {
    sample();
    part1();
    // part2();
}

fn sample() {
    println!("Sample: ");
    let problem = file_as_problem("src/sample.txt");
    println!("Towels: {:?}", &problem.towels);
    let mut count = 0;
    for pattern in &problem.patterns {
        let towels = matching_towels(&pattern, &problem.towels);
        println!("Pattern: {}\tTowels: {:?}", pattern, towels);
        if towels.len() > 0 { count += 1 }
    }
    println!("Count: {}", count);
}

 fn part1() {
    println!("Part One: ");
    let problem = file_as_problem("src/input.txt");
    let mut count = 0;
    for pattern in &problem.patterns {
        println!("Pattern: {}", pattern);
        let towels = matching_towels(&pattern, &problem.towels);
        println!("Pattern: {}\tTowels: {:?}", pattern, towels);
        if towels.len() > 0 { count += 1 }
    }
    println!("Count: {}", count);
}

fn part2() {
    println!("Part Two: ");
    let problem = file_as_problem("src/input.txt");
}

fn file_as_problem(path: &str) -> Problem {
    let mut problem = Problem {towels: vec![], patterns:vec![]};
    let contents = file_contents(path);
    let lines:Vec<&str> = contents.lines().collect();
    problem.towels = lines[0].split(", ").map(|s|s.to_string()).collect();
    for i in 2..lines.len() {
        problem.patterns.push(lines[i].trim().to_string());
    }
    return problem
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

#[derive (Debug)]
struct Problem {
    towels: Vec<String>,
    patterns: Vec<String>
}

fn matching_towels(pattern: &str, towels: &Vec<String>) -> Vec<String> {
    let mut matches = vec![];
    let subset = possible_towels(pattern, towels);
    // println!("Subset: {} of {}", subset.len(), towels.len());
    for towel in &subset {
        // if a towel matches the whole pattern, return that
        if pattern.eq(towel) {
            return vec![towel.to_string()]
        }
        // towel matches the start. See if the rest is matchable
        if pattern.starts_with(towel) {
            let rest = &pattern[towel.len()..];
            let mut towels = matching_towels(rest, &subset);
            // matches for the rest
            if towels.len() > 0 {
                matches = vec![towel.to_string()];
                matches.append(&mut towels);
            }
        }
    }
    return matches
}

fn possible_towels(pattern: &str, towels: &Vec<String>) -> Vec<String> {
    let mut possible = vec![];
    for towel in towels {
        if pattern.contains(towel) {
            possible.push(towel.to_string());
        }
    }
    return possible;
}
