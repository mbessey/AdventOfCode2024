
fn main() {
    sample();
    // part1();
    // part2();
}

fn sample() {
    println!("Sample Data");
    let equations = file_as_equations("src/sample.txt");
    pretty_print(&equations);
    for eq in equations {
        let ops = operations(&eq);
        if ops.len() > 0 {
            print!("{} = {}", eq.test_value, eq.operands[0]);
            for i in 0..ops.len() {
                print!(" {} {}", ops[i], eq.operands[i+1]);
            }
            println!()
        } else {
            println!("FAIL");
        }
    }
}

fn part1() {
    println!("Part 1");
}

fn part2() {
    println!("Part 2");
}

fn operations(e: &Equation) -> Vec<char> {
    let ops:Vec<char> = Vec::new();
    let test_value = e.test_value;
    let operands = &e.operands;
    let spaces = e.operands.len() - 1;
    if spaces == 1 {
        if operands[0] + operands[1] == test_value {
            return vec!['+']
        } else if operands[0] * operands[1] == test_value{
            return vec!['*']
        } else {
            return vec![]
        }
    } else {
        // first op could be *
        if test_value % operands[0] == 0 {
            let e2 = Equation {
                test_value: test_value / operands[0],
                operands: operands[1..].to_vec()
            };
            let mut sub_ops = operations(&e2);
            if sub_ops.len() > 0 {
                let mut result = vec!['*'];
                result.append(&mut sub_ops);
                return result
            }
        }
        // first op could be + 
        // this is always true, yeah?
        if test_value - operands[0] > 0 {
            let e2 = Equation {
                test_value: test_value / operands[0],
                operands: operands[1..].to_vec()
            };
            let mut sub_ops = operations(&e2);
            if sub_ops.len() > 0 {
                let mut result = vec!['+'];
                result.append(&mut sub_ops);
                return result
            }
        }
    }
    return vec![];
}

struct Equation {
    test_value: i64,
    operands: Vec<i64>
}

fn file_as_equations(path: &str) -> Vec<Equation> {
    let mut result = Vec::new();
    let contents = file_contents(path);
    let rows = contents.lines();
    for row in rows {
        let mut e = Equation {
            test_value: 0,
            operands: vec![]
        };
        let mut row_vec = Vec::new();
        let mut parts = row.split(":");
        e.test_value = parts.next().unwrap().parse().unwrap();
        let values = parts.next().unwrap().split_whitespace();
        for value in values {
            let v = value.parse::<i64>()
            .expect("Failed to parse");
            row_vec.push(v);
        }
        e.operands = row_vec;
        result.push(e);
    }
    return result;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}

fn pretty_print(vec:&Vec<Equation>) {
    for row in vec {
        println!("{}, {:?}", row.test_value, row.operands);
    }
}
