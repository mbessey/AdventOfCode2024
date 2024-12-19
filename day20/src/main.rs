type Number = i64;

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let mut file = file_contents("src/sample.txt");
}

 fn part1() {
    println!("Part One: ");
    let mut file = file_contents("src/input.txt");
}

fn part2() {
    println!("Part Two: ");
    let mut file = file_contents("src/input.txt");
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
