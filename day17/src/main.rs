use std::cmp::min;

use regex::Regex;

type Number = i64;
#[derive(Debug)]
struct Program {
    reg_a: Number,
    reg_b: Number,
    reg_c: Number,
    ip: usize,
    instructions: Vec<Number>
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
    let mut halted = false;
    while ! halted {
        halted = step(&mut program_state);
        println!("Program state: {:?}", program_state);
    }
    println!("Halted.");
}

fn part1() {
    println!("PART ONE: ");
    let mut program_state = file_as_program("src/part1.txt");
    println!("Program state: {:?}", program_state);
    let mut halted = false;
    while ! halted {
        halted = step(&mut program_state);
        // println!("Program state: {:?}", program_state);
    }
    println!("Halted.");
}

fn part2() {
    println!("PART TWO: ");
}

fn step(program: &mut Program) -> bool {
    let mut halted = false;
    let instruction = program.instructions[program.ip];
    program.ip += 1;
    match instruction {
        0 => { adv(program) }, // divide A / 2^combo => A
        1 => { bxl(program) }, // bitwise xor literal B|L => B
        2 => { bst(program) }, // mod 8 combo%8 => B
        3 => { jnz(program) }, // jump if non-zero
        4 => { bxc(program) }, // bitwise XOR b|c
        5 => { out(program) }, // output
        6 => { bdv(program) }, // divide A / 2^combo => B
        7 => { cdv(program) }, // divide A / 2^combo => C
        _ => {
            panic!("bad instruction: {}", instruction)
        }
    }
    if program.ip >= program.instructions.len() {
        halted = true;
    }
    return halted
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
    print!("{},", c % 8);
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
        instructions: Vec::new(),        
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
