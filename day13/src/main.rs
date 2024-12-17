use std::cmp::min;

use regex::Regex;

type Number = i64;
#[derive(Debug)]
struct Coord {
    x: Number,
    y: Number
}

#[derive(Debug)]
struct Game {
    a: Coord,
    b: Coord,
    prize: Coord,
}

#[derive(Debug)]
struct Solution {
    a_presses: Number,
    b_presses: Number,
}

impl Game {
    fn new() -> Game {
        Game{
            a: Coord { x: 0, y: 0 },
            b: Coord { x: 0, y: 0 },
            prize: Coord { x: 0, y: 0 },
        }
    }
}
fn main() {
    // sample();
    // part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let games = file_as_games("src/sample.txt");
    let mut total = 0;
    for game in games {
        // println!("{:?}", game);
        let solutions = solve_game(game);
        // println!("Solutions: {:?}", solutions);
        let lowest = lowest_cost(solutions);
        match lowest {
            Some(x) => {
                println!("Lowest cost: {}", x);
                total += x;
            },
            None => {
                println!("No solution");
            }
        }
    }
    println!("Total Cost: {}", total);
}

fn part1() {
    println!("PART ONE: ");
    let games = file_as_games("src/part1.txt");
    let mut total = 0;
    for game in games {
        // println!("{:?}", game);
        let solutions = solve_game(game);
        // println!("Solutions: {:?}", solutions);
        let lowest = lowest_cost(solutions);
        match lowest {
            Some(x) => {
                println!("Lowest cost: {}", x);
                total += x;
            },
            None => {
                println!("No solution");
            }
        }
    }
    println!("Total Cost: {}", total);
}

fn part2() {
    println!("PART TWO: ");
    let mut games = file_as_games("src/sample.txt");
    adjust_prizes(& mut games);
    let mut total = 0;
    for game in games {
        // println!("{:?}", game);
        let solutions = solve_game(game);
        // println!("Solutions: {:?}", solutions);
        let lowest = lowest_cost(solutions);
        match lowest {
            Some(x) => {
                println!("Lowest cost: {}", x);
                total += x;
            },
            None => {
                println!("No solution");
            }
        }
    }
    println!("Total Cost: {}", total);
}

fn adjust_prizes(games: & mut Vec<Game>) {
    for game in games {
        game.prize.x += 10000000000000;
        game.prize.y += 10000000000000;
    }
}

fn solve_game(game: Game) -> Vec<Solution> {
    let mut result = Vec::new();
    let a_limit = min(game.prize.x / game.a.x, game.prize.y/game.a.y);
    let b_limit = min(game.prize.x / game.b.x, game.prize.y/game.b.y);
    for a in 0..a_limit {
        for b in 0..b_limit {
            let x = a * game.a.x + b * game.b.x;
            let y = a * game.a.y + b * game.b.y;
            if x == game.prize.x && y == game.prize.y {
                result.push(Solution { a_presses: a, b_presses: b });
            }
        }
    }
    return result;
}

fn lowest_cost(solutions: Vec<Solution>) -> Option<Number> {
    if solutions.len() == 0 {
        return None
    }
    let mut lowest = cost(&solutions[0]);
    for solution in solutions {
        let cost = cost(&solution);
        if cost < lowest { lowest = cost }
    }
    return Some(lowest)
}

fn cost(solution: &Solution) -> Number {
    solution.a_presses * 3 + solution.b_presses
}

fn file_as_games(path: &str) -> Vec<Game> {
    let mut result = Vec::new();
    let contents = file_contents(path);
    let line_re = Regex::new(r"(Button A|Button B|Prize): X[+=]([0-9]+), Y[+=]([0-9]+)")
    .expect("Regex error");
    let mut game = Game::new();
    for capture in line_re.captures_iter(&contents) {
        let x = capture[2].parse::<Number>()
        .expect("can't parse X");
        let y = capture[3].parse::<Number>()
        .expect("can't parse X");
        match &capture[1] {
            "Button A" => {
                game.a = Coord{x: x, y: y};
            },
            "Button B" => {
                game.b = Coord{x: x, y: y};
            },
            "Prize" => {
                game.prize = Coord{x: x, y: y};
                result.push(game);
                game = Game::new();
            },
            _ => ()
        }
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
