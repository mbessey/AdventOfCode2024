
fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let values = file_as_vec("src/sample.txt");
    // pretty_print(&values);

}

fn part1() {
    println!("PART ONE: ");

}

fn part2() {
    println!("PART TWO: ");

}

fn file_as_vec(path: &str) -> Vec<char> {
    let contents = file_contents(path);
    return contents.trim().chars().collect();
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

fn pretty_print(vec:&Vec<char>) {
    println!("{:?}, ", vec);
}
