use std::path::MAIN_SEPARATOR;


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
