use std::{collections::{HashMap, HashSet}, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: i32,
    col: i32
}

#[derive(Debug)]
struct Map {
    antennae: HashMap<char, Vec<Coord>>,
    width: usize,
    height: usize
}

fn main() {
    // sample();
    part1();
    part2();
}

fn sample() {
    let map = file_as_map("src/sample.txt");
    println!("{:?}", &map);
    let nodes = get_nodes(&map);
    println!("{} Nodes: {:?}", nodes.len(), nodes)
}

fn part1() {
    let map = file_as_map("src/part1.txt");
    println!("Width: {}, Height: {}, antennae: {}", &map.width, &map.height, &map.antennae.len());
    let nodes = get_nodes(&map);
    println!("{} Nodes", nodes.len())
}

fn part2() {
    let map = file_as_map("src/part1.txt");
    println!("Width: {}, Height: {}, antennae: {}", &map.width, &map.height, &map.antennae.len());
    let nodes = get_nodes2(&map);
    println!("{} Nodes", nodes.len())
}

/*

    a: 1, 8
    b: 2, 5

    col_delta = 3
    row_delta = -1
 */

fn get_nodes(map: &Map) -> HashSet<Coord> {
    let mut result: HashSet<Coord> = HashSet::new();
    for (freq, antennae) in &map.antennae {
        println!("Frequency {}: {} antennae", freq, antennae.len());
        for (a,b) in pairwise(antennae) {
            let col_delta = a.col - b.col;
            let row_delta = a.row - b.row;
            let c = Coord {
                row: b.row - row_delta,
                col: b.col - col_delta
            };
            let d = Coord {
                row: a.row + row_delta,
                col: a.col + col_delta
            };
            if (c.row < map.height as i32 && c.row >= 0) && (c.col < map.width as i32 && c.col >= 0) {
                result.insert(c);
            }
            if (d.row < map.height as i32 && d.row >= 0) && (d.col < map.width as i32 && d.col >= 0) {
                result.insert(d);
            }
        }
    }
    return result;
}

fn get_nodes2(map: &Map) -> HashSet<Coord> {
    let mut result: HashSet<Coord> = HashSet::new();
    for (freq, antennae) in &map.antennae {
        println!("Frequency {}: {} antennae", freq, antennae.len());
        for (a,b) in pairwise(antennae) {
            result.insert(*a);
            result.insert(*b);            
            let col_delta = a.col - b.col;
            let row_delta = a.row - b.row;
            for m in 1..50 {
                let c = Coord {
                    row: b.row - row_delta * m,
                    col: b.col - col_delta * m
                };
                if (c.row < map.height as i32 && c.row >= 0) && (c.col < map.width as i32 && c.col >= 0) {
                    result.insert(c);
                }
            }
            for m in 1..50 {
                let d = Coord {
                    row: a.row + row_delta * m,
                    col: a.col + col_delta * m
                };
                if (d.row < map.height as i32 && d.row >= 0) && (d.col < map.width as i32 && d.col >= 0) {
                    result.insert(d);
                }
            }
        }
    }
    return result;
}

fn pairwise(vec: &Vec<Coord>) -> Vec<(&Coord, &Coord)> {
    let mut pairs = Vec::new();
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            pairs.push((&vec[i], &vec[j]));
        }    
    }
    return pairs;
}

fn file_as_map(path: &str) -> Map {
    let contents = file_contents(path);
    let mut rows = contents.lines();
    let mut result = Map {
        height: rows.clone().count(),
        width: rows.next().unwrap().len(),
        antennae: HashMap::new()
    };
    let mut rows = contents.lines();
    for row in 0..result.height {
        let line = rows.next().unwrap();
        let mut chars = line.chars();
        for col in 0..line.len() {
            let ch = chars.next().unwrap();
            if ch != '.' {
                if result.antennae.contains_key(&ch) {
                    let v2:&mut Vec<Coord> = result.antennae.get_mut(&ch).unwrap();
                    v2.push(Coord {row: row as i32, col: col as i32});
                } else {
                    result.antennae.insert(ch, vec![Coord {row: row as i32, col: col as i32}]);
                }
            }
        }
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
