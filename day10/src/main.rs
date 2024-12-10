use std::collections::HashSet;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: usize,
    col: usize
}

type Map = Vec<Vec<i32>>;

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let map = file_as_map("src/sample.txt");
    pretty_print(&map);
    let heads = find_trailheads(&map);
    //println!("{:?}", heads);
    let mut sum_scores = 0;
    let mut sum_ranks = 0;
    for head in heads {
        println!("For trailhead: {:?}", head);
        let summits = summits_from_head(&map, &head);
        let rank = rank_head(&map, &head);
        println!("{} summits, rank: {}", summits.len(), rank);
        sum_scores += summits.len();
        sum_ranks += rank;
    }
    println!("Total score: {}", sum_scores);
    println!("Total ranks: {}", sum_ranks);
}

fn part1() {
    println!("PART ONE: ");
    let map = file_as_map("src/part1.txt");
    //pretty_print(&map);
    let heads = find_trailheads(&map);
    //println!("{:?}", heads);
    let mut sum_scores = 0;
    for head in heads {
        println!("For trailhead: {:?}", head);
        let summits = summits_from_head(&map, &head);
        println!("{} summits", summits.len());
        sum_scores += summits.len();
    }
    println!("Total score: {}", sum_scores);
}

fn part2() {
    println!("PART TWO: ");
    let map = file_as_map("src/part1.txt");
    //pretty_print(&map);
    let heads = find_trailheads(&map);
    //println!("{:?}", heads);
    let mut sum_ranks = 0;
    for head in heads {
        println!("For trailhead: {:?}", head);
        let rank = rank_head(&map, &head);
        println!("rank: {}", rank);
        sum_ranks += rank;
    }
    println!("Total ranks: {}", sum_ranks);
}

fn summits_from_head(map: &Map, coords: &Coord) -> HashSet<Coord> {
    let height = map.len();
    let width = map[0].len();
    let mut summits = HashSet::new();
    let elevation = map[coords.row][coords.col];
    if elevation == 9 {
        // we've reached the summit. that's one peak
        summits.insert(coords.clone());
    } else {
        // Where can we go from here?
        let neighbors = neighbors(&coords, height, width);
        for neighbor in neighbors {
            // only go uphill by one
            if map[neighbor.row][neighbor.col] == elevation + 1 {
                let other = summits_from_head(map, &neighbor);
                for summit in other {
                    summits.insert(summit.clone());
                }
            }
        }
    }
    return summits;
}

fn rank_head(map: &Map, coords: &Coord) -> i32 {
    let height = map.len();
    let width = map[0].len();
    let mut rank = 0;
    let elevation = map[coords.row][coords.col];
    if elevation == 9 {
        // we've reached the summit. that's one peak
        // println!("Summit at: {:?}", coords);
        rank = 1;
    } else {
        // Where can we go from here?
        let neighbors = neighbors(&coords, height, width);
        for neighbor in neighbors {
            // only go uphill by one
            if map[neighbor.row][neighbor.col] == elevation + 1 {
                rank += rank_head(map, &neighbor);
            }
        }
    }
    return rank;
}

fn neighbors(coords: &Coord, height: usize, width: usize) -> Vec<Coord>{
    let mut neighbors = vec![];
    if coords.row > 0 { neighbors.push(Coord { row: coords.row-1, col: coords.col }); }
    if coords.row < height-1 { neighbors.push(Coord { row: coords.row+1, col: coords.col }); }
    if coords.col > 0 { neighbors.push(Coord { row: coords.row, col: coords.col - 1}); }
    if coords.col < width-1 { neighbors.push(Coord { row: coords.row, col: coords.col + 1}); }
    return neighbors;
}

fn find_trailheads(map: &Map) -> Vec<Coord> {
    let mut heads = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                heads.push(Coord{row: row, col: col});
            }
        }
    }
    return heads
}

fn file_as_map(path: &str) -> Map {
    let mut result = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        let mut row_vec:Vec<i32> = Vec::new();
        let values = row.chars();
        for value in values {
            row_vec.push(value as i32 - '0' as i32);
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

fn pretty_print(vec:&Map) {
    for row in vec {
        println!("{:?}, ", row);
    }
}
