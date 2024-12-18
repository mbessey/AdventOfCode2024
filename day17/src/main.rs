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
    check_output: bool,
    initial_a: Number,
    initial_b: Number,
    initial_c: Number,
}

impl Program {
    fn new(a: Number, b: Number, c: Number, instructions: Vec<Number>) -> Program {
        let len = instructions.len();
        Program { 
            reg_a: a,
            reg_b: b,
            reg_c: c, 
            ip: 0,
            halted: false,
            instructions: instructions,
            output: Vec::with_capacity(len),
            check_output: false,
            initial_a: a,
            initial_b: b,
            initial_c: c,
        }
    }

    fn reset(self: &mut Program) {
        self.reg_a = self.initial_a;
        self.reg_b = self.initial_b;
        self.reg_c = self.initial_c;
        self.ip = 0;
        self.halted = false;
        self.output.clear();
    }

    fn step(self: &mut Program) {
        let instruction = self.instructions[self.ip];
        self.ip += 1;
        match instruction {
            0 => { self.adv() }, // divide A / 2^combo => A
            1 => { self.bxl() }, // bitwise xor literal B|L => B
            2 => { self.bst() }, // mod 8 combo%8 => B
            3 => { self.jnz() }, // jump if non-zero
            4 => { self.bxc() }, // bitwise XOR b|c => B
            5 => { self.out() }, // output
            6 => { self.bdv() }, // divide A / 2^combo => B
            7 => { self.cdv() }, // divide A / 2^combo => C
            _ => {
                panic!("bad instruction: {}", instruction)
            }
        }
        if self.ip >= self.instructions.len() {
            self.halted = true;
        }
    }
    
    fn adv(self: &mut Program) {
        let c = self.combo_operand();
        let result = self.reg_a / (1 << c);
        self.reg_a = result;
    }
    
    fn bxl(self: &mut Program) {
        let l = self.literal_operand();
        self.reg_b = self.reg_b ^ l;
    }
    
    fn bst(self: &mut Program) {
        let c = self.combo_operand();
        self.reg_b = c % 8;
    }
    
    fn jnz(self: &mut Program) {
        let target = self.literal_operand();
        if self.reg_a != 0 {
            self.ip = target as usize
        }
    }
    
    fn bxc(self: &mut Program) {
        let _ = self.literal_operand();
        let b = self.reg_b;
        let c = self.reg_c;
        self.reg_b = b ^ c;
    }
    
    fn out(self: &mut Program) {
        let c = self.combo_operand();
        let v = c % 8;
        self.output.push(v);
        if self.check_output {
            if v != self.instructions[self.output.len()-1] {
                self.halted = true;
            }
        }
    }
    
    fn bdv(self: &mut Program) {
        let c = self.combo_operand();
        let result = self.reg_a / (1 << c);
        self.reg_b = result;
    }
    
    fn cdv(self: &mut Program) {
        let c = self.combo_operand();
        let result = self.reg_a / (1 << c);
        self.reg_c = result;
    }
    
    fn literal_operand(self: &mut Program) -> Number {
        let operand = self.instructions[self.ip];
        self.ip += 1;
        return operand
    }
    
    fn combo_operand(self: &mut Program) -> Number {
        let operand = self.instructions[self.ip];
        self.ip += 1;
        match operand {
            0..=3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("invalid combo operand")
        }
    }
    
    fn run(self: &mut Program) {
        while ! self.halted {
            self.step();
            // println!("Program state: {:?}", program_state);
        }
    }
}

fn main() {
    // sample();
    // part1();
    // part2();
    // instruction_tests();
    sample2();
}

fn sample() {
    println!("Sample: ");
    let mut program_state = file_as_program("src/sample.txt");
    println!("Program state: {:?}", program_state);
    while ! program_state.halted {
        program_state.step();
        println!("Program state: {:?}", program_state);
    }
    println!("Halted.");
}

fn part1() {
    println!("PART ONE: ");
    let mut program_state = file_as_program("src/part1.txt");
    println!("Program state: {:?}", program_state);
    program_state.run();
    println!("Halted.");
    let output_strings:Vec<String> = program_state.output.iter().map(|x|format!("{}", x)).collect();
    println!("{}", output_strings.join(","));
}

fn sample2() {
    println!("Sample2: ");
    let mut program = file_as_program("src/part1.txt");
    println!("target output: {:?}", program.instructions);
    for a in 0..1000 {
        program.reset();
        program.reg_a = a;
        program.run();
        if prefix_match(&program.output, &program.instructions) {
            println!("{:o} Program output: {:?}", a, program.output);
        } else {
            println!("{:?}", program.output);
        }
    }
    println!("Halted.");
}

fn instruction_tests() {
    println!("Instruction tests from web:");
    // If register C contains 9, the program 2,6 would set register B to 1.
    let mut program = Program::new(0, 0, 9, vec![2,6]);
    program.run();
    assert!(program.reg_b == 1);
    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    let mut program = Program::new(10, 0, 0, vec![5,0,5,1,5,4]);
    program.run();
    assert!(program.output == vec![0,1,2]);
    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    let mut program = Program::new(2024, 0, 0, vec![0,1,5,4,3,0]);
    program.run();
    assert!(program.output == vec![4,2,5,6,7,7,7,7,3,1,0]);
    assert!(program.reg_a == 0);
    // If register B contains 29, the program 1,7 would set register B to 26.
    let mut program = Program::new(0, 29, 0, vec![1, 7]);
    program.run();
    assert!(program.reg_b == 26);
    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    let mut program = Program::new(0, 2024, 43690, vec![4, 0]);
    program.run();
    assert!(program.reg_b == 44354);
}

fn prefix_match(output: &[Number], instructions: &[Number]) -> bool {
    for i in 0..output.len() {
        if output[i] != instructions[i] {
            return false;
        }
    }
    return true;
}

fn part2() {
    println!("PART TWO: ");
    let mut program_state = file_as_program("src/part1.txt");
    let mut start_a = 10473000000;
    program_state.check_output = true;
    loop {
        program_state.reg_a = start_a;
        if start_a % 1000000 == 0 {
            println!("reg_a == {}", program_state.reg_a);
        }
        program_state.run();
        if program_state.output == program_state.instructions {
            println!("Output matched with a = {}", start_a);
            break;
        }
        start_a += 1;
        program_state.reset();
    }
}

fn file_as_program(path: &str) -> Program {
    let contents = file_contents(path);
    let reg_re = Regex::new(r"(Register A|Register B|Register C): ([0-9]+)")
    .expect("Regex error");
    let program_re = Regex::new(r"Program: ([0-9]+(,[0-9]+)*)")
    .expect("Regex error");
    let mut a =0;
    let mut b= 0;
    let mut c =0;
    let mut i = vec![];
    
    for line in contents.lines() {
        if let Some(register) = reg_re.captures(line) {
            // println!("reg: {:?}", register);
            let value = register[2].parse::<Number>()
            .expect("parse error");
            match &register[1] {
                "Register A" => a = value,
                "Register B" => b = value,
                "Register C" => c = value,
                _ => { panic!() }
            }
        } else if let Some(program) = program_re.captures(line) {
            // println!("prog: {:?}", program);
            let instructions:Vec<&str> = program[1].split(",").collect();
            i = instructions.into_iter().map(|s| s.parse::<Number>().unwrap()).collect();
        } else {
            //println!("Ignoring line: {}", line);
        }
    }
    return Program::new(a, b, c, i);
}

fn file_contents(path: &str) -> String {
    use std::fs;
    return fs::read_to_string(path)
        .expect("should have read the file");
}
