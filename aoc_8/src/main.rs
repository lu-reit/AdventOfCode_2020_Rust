use std::fs;
use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let mut program = Program::new(read_opcode("input"));

    let p1_timer = Instant::now();
    let p1_result = part1(&mut program);
    let p1_time = p1_timer.elapsed();
    println!("Result part1: {}", p1_result);
    println!("Time part1: {:?}", p1_time);

    program.reset(0, 0);

    let p2_timer = Instant::now();
    let p2_result = part2(&mut program);
    let p2_time = p2_timer.elapsed();
    println!("Result part2: {}", p2_result);
    println!("Time part2: {:?}", p2_time);
}

fn part2(program: &mut Program) -> i32 { 
    let mut has_code_run = vec![false; program.opcodes.len()];
    let (_, fail_history) = run_until_inf(program, &mut has_code_run);

    for (old_inst, old_acc) in fail_history.iter() {
        // Set the program and the tracker to the old state
        program.reset(*old_inst, *old_acc);
        has_code_run[*old_inst] = false;

        // Swap the instruction, ignore acc and run the program
        if let OpCode::Acc(_) = program.opcodes[program.inst] { continue; }
        program.swap_op(*old_inst);
        let (seen_inf, history) = run_until_inf(program, &mut has_code_run);

        if seen_inf {
            // Undo the changes to the tracker and the opcode
            history.iter().for_each(|(inst, _)| has_code_run[*inst] = false);
            program.swap_op(*old_inst);
        } else { break; }
    }
    program.acc
}

fn part1(program: &mut Program) -> i32 {
    let mut has_code_run = vec![false; program.opcodes.len()];
    let (seen_inf, _) = run_until_inf(program, &mut has_code_run);
    
    // If we did not encounter inf something has seriously went wrong
    if seen_inf { program.acc }
    else { panic!("Did not encounter infinite loop in part1"); }
}

// Runs a program until it finishes or a inf. loop is encountered.
// Keeps track of the state of the program for each instruction.
// Returns (true, history) if a inf. loop has been encountered.
// has_code_run keeps track whether the instruction at the current pointer
// has already been excecuted 
fn run_until_inf(program: &mut Program, has_code_run: &mut Vec<bool>) 
    -> (bool, VecDeque<(usize, i32)>) {
    let mut history: VecDeque<(usize, i32)> = VecDeque::new();

    loop {
        // Quit either if we encountered the instruction already 
        // or the instruction pointer points past the last instruction
        if program.inst == program.opcodes.len() { return (false, history); }
        if has_code_run[program.inst] { return (true, history); }

        has_code_run[program.inst] = true;
        history.push_front((program.inst, program.acc));
        program.run_once();
    }
}


// -------- The OpCode Interpreter -------- 

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Acc(i32),
    Nop(i32),
    Jmp(i32)
}

#[derive(Debug, Clone)]
struct Program {
    inst: usize, // Pointer to next opcode to be executed
    acc: i32, 
    opcodes: Vec<OpCode>
}

impl Program {
    fn new(opcodes: Vec<OpCode>) -> Program {
        Program {
            inst: 0, 
            acc: 0,
            opcodes: opcodes 
        }
    }

    // Run a single instruction and return 
    #[inline(always)]
    fn run_once(&mut self) {
        match self.opcodes[self.inst] {
            OpCode::Acc(x) => { self.acc += x; self.inst += 1; }
            OpCode::Nop(_) => { self.inst += 1; }
            OpCode::Jmp(x) => { self.inst = (self.inst as i32 + x) as usize }
        }
    }

    // Restart the program (does not reset the opcodes)
    fn reset(&mut self, old_inst: usize, old_acc: i32) {
        self.inst = old_inst;
        self.acc = old_acc;
    }

    // Swaps Jmp with Nop for the opcode at position
    fn swap_op(&mut self, position: usize) {
        let opcode: &mut OpCode = &mut self.opcodes[position];
        *opcode = match opcode {
            OpCode::Nop(x) => OpCode::Jmp(*x),
            OpCode::Jmp(x) => OpCode::Nop(*x),
            OpCode::Acc(x) => OpCode::Acc(*x)
        };
    }
}

// Parsing EUGHHHHHHHHH
fn read_opcode(filename: &str) -> Vec<OpCode> {
    let codes: Vec<String> = fs::read_to_string(filename).unwrap()
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut opcodes: Vec<OpCode> = Vec::with_capacity(codes.len());
    for code in codes {
        let mut parts = code.split(" ");
        let op = parts.next().expect("Failed to read Op");
        let num: i32 = match parts.next() {
            Some(s) if s.starts_with("+") => s[1..].parse().unwrap(),
            Some(s) => s.parse().unwrap(),
            None => panic!("Op has no associated value")
        };
        opcodes.push( match op {
            "acc" => OpCode::Acc(num),
            "jmp" => OpCode::Jmp(num),
            "nop" => OpCode::Nop(num),
            _ => panic!("Unknown OpCode encountered")
        });
    }
    opcodes
}
