use std::cmp::min;

use regex::Regex;

type Number = i64;
#[derive(Debug, Clone)]
struct Program {
    reg_a: Number,
    reg_b: Number,
    reg_c: Number,
    ip: usize,
    halted: bool,
    instructions: Vec<Number>,
    output: Vec<Number>,
    check_output: bool
}

fn main() {
    sample();
    part1();
    part2();
}

fn sample() {
    println!("Sample: ");
    let mut program_state = file_as_program("src/sample.txt");
    println!("Program state: {:?}", program_state);
    while ! program_state.halted {
        step(&mut program_state);
        println!("Program state: {:?}", program_state);
    }
    println!("Halted.");
}

fn part1() {
    println!("PART ONE: ");
    let mut program_state = file_as_program("src/part1.txt");
    println!("Program state: {:?}", program_state);
    run(&mut program_state);
    println!("Halted.");
    let output_strings:Vec<String> = program_state.output.iter().map(|x|format!("{}", x)).collect();
    println!("{}", output_strings.join(","));
}

fn part2() {
    println!("PART TWO: ");
    let program_state = file_as_program("src/part1.txt");
    let mut start_a = 2381000000;
    loop {
        let mut state = program_state.clone();
        state.check_output = true;
        state.reg_a = start_a;
        if start_a % 1000000 == 0 {
            println!("reg_a == {}", state.reg_a);
        }
        run(&mut state);
        if state.output == state.instructions {
            println!("Output matched with a = {}", start_a);
            break;
        }
        start_a += 1;
    }
}

fn run(program: &mut Program) {
    while ! program.halted {
        step(program);
        // println!("Program state: {:?}", program_state);
    }
}

fn step(program: &mut Program) {
    let instruction = program.instructions[program.ip];
    program.ip += 1;
    match instruction {
        0 => { adv(program) }, // divide A / 2^combo => A
        1 => { bxl(program) }, // bitwise xor literal B|L => B
        2 => { bst(program) }, // mod 8 combo%8 => B
        3 => { jnz(program) }, // jump if non-zero
        4 => { bxc(program) }, // bitwise XOR b|c => B
        5 => { out(program) }, // output
        6 => { bdv(program) }, // divide A / 2^combo => B
        7 => { cdv(program) }, // divide A / 2^combo => C
        _ => {
            panic!("bad instruction: {}", instruction)
        }
    }
    if program.ip >= program.instructions.len() {
        program.halted = true;
    }
}

fn adv(prog: &mut Program) {
    let c = combo_operand(prog);
    let result = prog.reg_a / (1 << c);
    prog.reg_a = result;
}

fn bxl(prog: &mut Program) {
    let l = literal_operand(prog);
    prog.reg_b = prog.reg_b ^ l;
}

fn bst(prog: &mut Program) {
    let c = combo_operand(prog);
    prog.reg_b = c % 8;
}

fn jnz(prog: &mut Program) {
    let target = literal_operand(prog);
    if prog.reg_a != 0 {
        prog.ip = target as usize
    }
}

fn bxc(prog: &mut Program) {
    let _ = literal_operand(prog);
    let b = prog.reg_b;
    let c = prog.reg_c;
    prog.reg_b = b ^ c;
}

fn out(prog: &mut Program) {
    let c = combo_operand(prog);
    let v = c % 8;
    prog.output.push(v);
    if prog.check_output {
        if v != prog.instructions[prog.output.len()-1] {
            prog.halted = true;
        }
    }
}

fn bdv(prog: &mut Program) {
    let c = combo_operand(prog);
    let result = prog.reg_a / (1 << c);
    prog.reg_b = result;
}

fn cdv(prog: &mut Program) {
    let c = combo_operand(prog);
    let result = prog.reg_a / (1 << c);
    prog.reg_c = result;
}

fn literal_operand(program: &mut Program) -> Number {
    let operand = program.instructions[program.ip];
    program.ip += 1;
    return operand
}

fn combo_operand(program: &mut Program) -> Number {
    let operand = program.instructions[program.ip];
    program.ip += 1;
    match operand {
        0..=3 => operand,
        4 => program.reg_a,
        5 => program.reg_b,
        6 => program.reg_c,
        _ => panic!("invalid combo operand")
    }
}

fn file_as_program(path: &str) -> Program {
    let contents = file_contents(path);
    let reg_re = Regex::new(r"(Register A|Register B|Register C): ([0-9]+)")
    .expect("Regex error");
    let program_re = Regex::new(r"Program: ([0-9]+(,[0-9]+)*)")
    .expect("Regex error");
    let mut prog = Program {
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        ip: 0,
        halted: false,
        instructions: Vec::new(),
        output: Vec::new(),
        check_output: false
    };
    for line in contents.lines() {
        if let Some(register) = reg_re.captures(line) {
            // println!("reg: {:?}", register);
            let value = register[2].parse::<Number>()
            .expect("parse error");
            match &register[1] {
                "Register A" => prog.reg_a = value,
                "Register B" => prog.reg_b = value,
                "Register C" => prog.reg_c = value,
                _ => { panic!() }
            }
        } else if let Some(program) = program_re.captures(line) {
            // println!("prog: {:?}", program);
            let instructions:Vec<&str> = program[1].split(",").collect();
            prog.instructions = instructions.into_iter().map(|s| s.parse::<Number>().unwrap()).collect();
        } else {
            println!("Ignoring line: {}", line);
        }
    }
    return prog;
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
