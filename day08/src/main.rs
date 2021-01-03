#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

use crate::Instruction::*;

#[derive(Debug)]
struct State {
    acc: i32,
    ip: usize,
    run_count: Vec<u32>,
}

fn main() {
    let mut ins = parse(include_str!("../input.txt"));
    let state = exec_until_loop(&ins);
    //part 1
    println!("Acc before loop is: {}", state.acc);
    //part 2
    let state = find_bad_ins(&mut ins).expect("No result found");
    println!("Acc at end of code: {}", state.acc);
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut prog = Vec::new();
    for l in input.lines() {
        let mut words = l.split_ascii_whitespace();
        let ins = match words.next() {
            Some("acc") => Acc(words.next().expect("ins argument").parse().expect("number")),
            Some("jmp") => Jmp(words.next().expect("jmp argument").parse().expect("number")),
            Some("nop") => Nop(words.next().expect("nop argument").parse().expect("number")),
            Some(x) => panic!("Unknown arg {}", x),
            None => panic!("no more ins"),
        };
        prog.push(ins)
    }
    prog
}

fn exec_until_loop(program: &[Instruction]) -> State {
    let mut s = State {
        ip: 0,
        acc: 0,
        run_count: vec![0; program.len()],
    };
    loop {
        if s.ip >= s.run_count.len() || s.run_count[s.ip] == 1 {
            break;
        }
        s.run_count[s.ip] += 1;
        match program[s.ip] {
            Acc(x) => {
                s.acc += x;
                s.ip += 1;
            }
            Jmp(x) => {
                s.ip = (s.ip as i32 + x) as usize;
            }
            Nop(_) => {
                s.ip += 1;
            }
        }
    }
    s
}

fn find_bad_ins(program: &mut [Instruction]) -> Option<State> {
    let mut i = 0;
    let s = exec_until_loop(program);
    if s.ip == program.len() {
        return Some(s);
    }
    // who cares about O(n^2) ? not I
    while i < program.len() {
        match program[i] {
            Jmp(x) => program[i] = Nop(x),
            Nop(x) => program[i] = Jmp(x),
            Acc(_) => {
                i += 1;
                continue;
            }
        };
        let s = exec_until_loop(program);
        if s.ip == program.len() {
            return Some(s);
        }
        // swap again
        match program[i] {
            Jmp(x) => program[i] = Nop(x),
            Nop(x) => program[i] = Jmp(x),
            Acc(_) => panic!("Unreachable code"),
        };
        i += 1;
    }
    None
}

#[test]
fn test() {
    let mut prog = parse(include_str!("../sample.txt"));
    let state = exec_until_loop(&prog);
    assert_eq!(state.acc, 5);
    let state = find_bad_ins(&mut prog).unwrap();
    assert_eq!(state.acc, 8);
}
