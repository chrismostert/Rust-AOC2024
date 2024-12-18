use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Program {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    ops: Vec<u64>,
    out: Vec<u64>,
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|w| !w.is_empty())
            .map(|d| d.parse::<u64>().context("Could not map digit"));

        Ok(Program {
            a: parts.next().context("No element")??,
            b: parts.next().context("No element")??,
            c: parts.next().context("No element")??,
            pc: 0,
            ops: parts.try_collect()?,
            out: Vec::new(),
        })
    }
}

impl Program {
    fn combo(&self, op: u64) -> u64 {
        match op {
            lit @ 0..=3 => lit,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            other => panic!("Invalid combo operator {}", other),
        }
    }

    fn run(&self, a_val: u64) -> Vec<u64> {
        let mut prog = self.clone();
        prog.a = a_val;
        while prog.pc < prog.ops.len() {
            let (instruction, op) = (prog.ops[prog.pc], prog.ops[prog.pc + 1]);
            match instruction {
                0 => prog.a /= 2u64.pow(prog.combo(op) as u32),
                1 => prog.b ^= op,
                2 => prog.b = prog.combo(op) % 8,
                3 => {
                    if prog.a != 0 {
                        prog.pc = op as usize;
                        continue;
                    }
                }
                4 => prog.b ^= prog.c,
                5 => prog.out.push(prog.combo(op) % 8),
                6 => prog.b = prog.a / 2u64.pow(prog.combo(op) as u32),
                7 => prog.c = prog.a / 2u64.pow(prog.combo(op) as u32),
                err => panic!("No such instruction {err}",),
            }
            prog.pc += 2;
        }
        prog.out
    }
}

fn find_quine(program: &Program, scan_start: u64, op_idx: usize) -> Option<u64> {
    for a_val in scan_start..scan_start + 8 {
        if program.run(a_val)[0] == program.ops[op_idx] {
            if op_idx == 0 {
                return Some(a_val);
            }
            if let Some(next_scan_start) = find_quine(program, a_val * 8, op_idx - 1) {
                return Some(next_scan_start);
            }
        }
    }
    None
}

fn main() {
    let program = include_str!("../../inputs/day17.txt")
        .parse::<Program>()
        .unwrap();

    println!("Part 1: {}", program.run(program.a).iter().join(","));
    println!(
        "Part 2: {}",
        find_quine(&program, 0, program.ops.len() - 1).unwrap()
    );
}
