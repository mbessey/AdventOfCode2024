use std::{path::MAIN_SEPARATOR, sync::LockResult};


type Number = i64;
type Map = Vec<Option<Number>>;

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let mut map = file_as_map("src/sample.txt");
    println!("{:?}", &map);
    pretty_print(&map);
    compact(&mut map);
    pretty_print(&map);
    println!("Checksum: {}", checksum(&map));
}

fn part1() {
    println!("PART ONE: ");
    let mut map = file_as_map("src/part1.txt");
    println!("Map length: {}", map.len());
    // pretty_print(&map);
    compact(&mut map);
    // pretty_print(&map);
    println!("Checksum: {}", checksum(&map));
}

fn part2() {
    println!("PART TWO: ");
    let mut map = file_as_map("src/part1.txt");
    println!("Map length: {}", map.len());
    // pretty_print(&map);
    compact2(&mut map);
    // pretty_print(&map);
    println!("Checksum: {}", checksum(&map));
}

fn checksum(map: &Map) -> Number {
    let mut cksum = 0;
    for block in 0..map.len() {
        match map[block] {
            None => (),
            Some(x) => { cksum += block as Number * x as Number }
        }
    }
    return cksum;
}

fn compact(map: &mut Map) {
    let mut left = 0;
    let mut right = map.len() - 1;
    while right > left {
        if map[left] == None && map[right] != None {
            let temp = map[right];
            map[right] = map[left];
            map[left] = temp;
        }
        if map[right] == None {
            right -= 1;
        }
        if map[left] != None {
            left += 1;
        }
    }
}

fn compact2(map: &mut Map) {
    // Find highest-numbered file
    // Find leftmost empty space it could fit
    // Move file
    // Rduce fileID
    let mut file_id = max_file(&map);
    while file_id > 0 {
        let (file_location, size) = find_file(&map, file_id);
        let hole_location = find_hole(&map, size);
        if hole_location != None {
            let hole = hole_location.unwrap();
            if hole < file_location {
                move_file(map, file_location, hole_location.unwrap(), size);
            }
        }
        file_id -= 1;
    }
}

fn max_file(map: &Map) -> Number {
    let mut max = 0;
    for i in 0..map.len() {
        if map[i] > Some(max) {
            max = map[i]
            .expect("Map subscript failed")
        }
    }
    return max;
}

fn find_file(map: &Map, file_id: Number) -> (usize, usize) {
    let mut size = 0;
    let mut location = 0;
    for i in (0..map.len()).rev() {
        if map[i] == Some(file_id) {
            location = i;
            size += 1;
        }
    }
    return (location, size)
}

fn find_hole(map: &Map, size: usize) -> Option<usize> {
    let mut location = None;
    let mut hole_size = 0;
    for i in 0..map.len() {
        if map[i] == None {
            hole_size += 1;
            if location == None {
                location = Some(i);
            }
            if hole_size == size {
                return location
            }
        } else {
            location = None;
            hole_size = 0;
        } 
    }
    return None;
}

fn move_file(map: &mut Map, file_location: usize, hole_location: usize, size: usize) {
    for i in 0..size {
        map[hole_location+i] = map[file_location+i];
        map[file_location+i] = None;
    }
}

fn file_as_map(path: &str) -> Map {
    let mut map = Map::new();
    let contents = file_contents(path);
    let mut is_file = true;
    let mut file_id = 0;
    for digit in contents.trim().chars() {
        let length = digit as Number - '0' as Number;
        let value: Option<Number>;
        if is_file {
            value = Some(file_id);
            file_id += 1;
        } else {
            value = None
        }
        for _i in 0..length {
            map.push(value);
        }
        is_file = !is_file;
    }
    return map;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

fn pretty_print(map: &Map) {
    for block in map {
        match block {
            Some(x) => print!("{}", x),
            None => print!(".")
        }
    }
    println!("")
}
