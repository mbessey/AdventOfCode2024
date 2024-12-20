use std::thread;


fn main() {
    // sample();
    // part1();
    part2();
}

fn sample() {
    println!("Sample Data");
    let equations = file_as_equations("src/sample.txt");
    // pretty_print(&equations);
    solve(&equations, true, false);
}

fn part1() {
    println!("Part 1");
    let equations = file_as_equations("src/part1.txt");
    // pretty_print(&equations);
    solve(&equations, false, false);
}

fn part2() {
    println!("Part 2");
    let equations = file_as_equations("src/part1.txt");
    // pretty_print(&equations);
    solve(&equations, true, false);
}

fn solve(equations: &Vec<Equation>, allow_concat:bool, print: bool) {
    let mut total = 0;
    for eq in equations {
        let ops = operations(&eq, allow_concat);
        if ops.len() > 0 {
            if print { print!("{} = {}", eq.test_value, eq.operands[0]); };
            for i in 0..ops.len() {
                if print {print!(" {} {}", ops[i], eq.operands[i+1]);}
            }
            if print { println!();}
            total += eq.test_value;
        } else {
            if print {println!("FAIL");}
        }
    }
    println!("TOTAL: {}", total);    
}

fn operations(e: &Equation, allow_concat:bool) -> Vec<char> {
    let test_value = e.test_value;
    let operands = &e.operands;
    let spaces = e.operands.len() - 1;
    if spaces == 1 {
        if operands[0] + operands[1] == test_value {
            return vec!['+']
        } else if operands[0] * operands[1] == test_value{
            return vec!['*']
        } else {
            if allow_concat {
                let s = format!("{}{}", operands[0], operands[1]);
                let appended = s.parse::<i64>().unwrap();
                if appended == test_value {
                    return vec!['|']
                }
            }
            return vec![]
        }
    } else {
        // first op could be *
        let new_op = operands[0] * operands [1];
        let mut ops = vec![new_op];
        ops.extend(operands[2..].iter());
        let e2 = Equation {
            test_value: test_value,
            operands: ops,
        };
        let mut sub_ops = operations(&e2, allow_concat);
        if sub_ops.len() > 0 {
            let mut result = vec!['*'];
            result.append(&mut sub_ops);
            return result
        }
        // first op could be +
        let new_op = operands[0] + operands [1];
        let mut ops = vec![new_op];
        ops.extend(operands[2..].iter());
        let e2 = Equation {
            test_value: test_value,
            operands: ops
        };
        let mut sub_ops = operations(&e2, allow_concat);
        if sub_ops.len() > 0 {
            let mut result = vec!['+'];
            result.append(&mut sub_ops);
            return result
        }
        // the concat operation...
        if allow_concat {
            let combined_ops = format!("{}{}", operands[0], operands[1]);
            let new_op = combined_ops.parse::<i64>().unwrap();
            let mut ops = vec![new_op];
            ops.extend(operands[2..].iter());
            let e2 = Equation {
                test_value: test_value,
                operands: ops
            };
            let mut sub_ops = operations(&e2, allow_concat);
            if sub_ops.len() > 0 {
                let mut result = vec!['|'];
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
