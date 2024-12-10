
#[derive(Debug)]
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
    println!("{:?}", heads);
    let mut sum_scores = 0;
    for head in heads {
        sum_scores += score_head(&map, &head);
    }
    println!("Total score: {}", sum_scores);
}

fn part1() {
    println!("PART ONE: ");

}

fn part2() {
    println!("PART TWO: ");

}

fn score_head(map: &Map, coords: &Coord) -> i32 {
    let height = map.len();
    let width = map[0].len();
    let mut score = 0;
    let elevation = map[coords.row][coords.col];
    if elevation == 9 {
        // we've reached the summit. that's one peak
        println!("Summit at: {:?}", coords);
        score = 1;
    } else {
        // Where can we go from here?
        let neighbors = neighbors(&coords, height, width);
        for neighbor in neighbors {
            // only go uphill by one
            if map[neighbor.row][neighbor.col] == elevation + 1 {
                score += score_head(map, coords);
            }
        }
    }
    return score;
}

fn neighbors(coords: &Coord, height: usize, width: usize) -> Vec<Coord>{
    let mut neighbors = vec![];
    let min_row = coords.row.saturating_sub(1);
    let min_col = coords.col.saturating_sub(1);
    let max_row = if coords.row == height-1 {coords.row} else {coords.row+1};
    let max_col = if coords.col == width-1 {coords.col} else {coords.col+1};
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if r == coords.row && c == coords.col {
                continue;
            }
            neighbors.push(Coord { row: r, col: c });
        }
    }
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
