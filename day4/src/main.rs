
fn main() {
    let values = file_as_vec2("src/test.txt");
    pretty_print(&values);
    println!("{}", row_solution(&values[0]));
    part1();
    part2();
}

fn row_solution(values: &Vec<i32>) -> String {
    return "42".to_string();
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
