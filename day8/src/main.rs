use std::{collections::HashMap};

#[derive(Debug)]
struct Coord {
    row: i32,
    col: i32
}

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    let values = file_as_vec2("src/sample.txt");
    let height = values.len() as i32;
    let width = values[0].len() as i32;
    //pretty_print(&values);
    let antennas = get_antennas(&values);
    print_antennas(&antennas);
    let nodes = get_nodes(&antennas, width, height);
    println!("Nodes: {:?}", nodes);
}

fn part1() {

}

fn part2() {

}

fn get_nodes(map: &HashMap<char, Vec<Coord>>, width:i32, height: i32) -> Vec<Coord> {
    let mut nodes = Vec::new();
    for (freq, coords) in map {
        print
    }
    return nodes;
}

fn print_antennas(map: &HashMap<char, Vec<Coord>>) {
    for key in map.keys() {
        println!("{}: {:?}", key, map.get(key).unwrap());
    }
}

fn get_antennas(v: &Vec<Vec<char>>) -> HashMap<char, Vec<Coord>> {
    let mut h = HashMap::new();
    for row in 0..v.len() {
        let row_vec = &v[row];
        for col in 0..row_vec.len() {
            let ch = row_vec[col];
            if ch != '.' {
                if h.contains_key(&ch) {
                    let v2:&mut Vec<Coord> = h.get_mut(&ch).unwrap();
                    v2.push(Coord {row: row as i32, col: col as i32});
                } else {
                    h.insert(ch, vec![Coord {row: row as i32, col: col as i32}]);
                }
            }
        }
    }
    return h;
}

fn file_as_vec2(path: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        let mut row_vec:Vec<char> = Vec::new();
        let values = row.chars();
        for value in values {
            row_vec.push(value);
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

fn pretty_print(vec:&Vec<Vec<char>>) {
    for row in vec {
        println!("{:?}, ", row);
    }
}
