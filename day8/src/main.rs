
fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    let values = file_as_vec2("src/sample.txt");
    pretty_print(&values);
}

fn part1() {

}

fn part2() {

}

fn file_as_vec2(path: &str) -> Vec<Vec<i32>> {
    let mut result:Vec<Vec<i32>> = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        let mut row_vec:Vec<i32> = Vec::new();
        let values = row.split_whitespace();
        for value in values {
            let v = value.parse::<i32>()
            .expect("Failed to parse");
            row_vec.push(v);
        }
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
