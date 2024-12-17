#![allow(non_camel_case_types)]
use super::{AOCError, Result};
use std::{fmt::Display, fs, path::PathBuf};

pub fn _main(data: PathBuf, verbosity: u8) -> Result<()> {
    let (mut cpu, stack) = parse(data)?;
    if verbosity > 1 {
        println!("{}  {:#?}", cpu, stack);
    }
    loop {
        let next_ins = cpu.fetch_op(&stack);
        if verbosity > 2 {
            println!("{:#?}", next_ins);
        }
        if next_ins == Instruction::halt {
            break;
        }
        cpu.execute_op(next_ins);
        if verbosity > 3 {
            println!("{}", cpu);
        }
    }
    cpu.flush();
    Ok(())
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Instruction {
    adv(u8),
    bxl(u8),
    bst(u8),
    jnz(u8),
    bxc(u8),
    out(u8),
    bdv(u8),
    cdv(u8),
    halt,
}

type Stack = Vec<Instruction>;

#[derive(Default, Debug)]
struct Cpu {
    stack_pointer: usize,
    register_a: u64,
    register_b: u64,
    register_c: u64,
    out_buf: Vec<u64>,
}

impl Cpu {
    fn execute_op(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::adv(op) => {
                self.register_a /= 2_u64.pow(self.combo_op(op) as u32);
                self.stack_pointer += 1;
            }
            Instruction::bxl(op) => {
                self.register_b ^= op as u64;
                self.stack_pointer += 1;
            }
            Instruction::bst(op) => {
                self.register_b = self.combo_op(op) % 8;
                self.stack_pointer += 1;
            }
            Instruction::jnz(op) => {
                if self.register_a != 0 {
                    self.stack_pointer = op as usize / 2;
                } else {
                    self.stack_pointer += 1;
                }
            }
            Instruction::bxc(_op) => {
                self.register_b ^= self.register_c;
                self.stack_pointer += 1;
            }
            Instruction::out(op) => {
                self.out_buf.push(self.combo_op(op) % 8);
                self.stack_pointer += 1;
            }
            Instruction::bdv(op) => {
                self.register_b = self.register_a / 2_u64.pow(self.combo_op(op) as u32);
                self.stack_pointer += 1;
            }
            Instruction::cdv(op) => {
                self.register_c = self.register_a / 2_u64.pow(self.combo_op(op) as u32);
                self.stack_pointer += 1;
            }
            Instruction::halt => {}
        }
    }

    fn fetch_op(&self, stack: &Stack) -> Instruction {
        if let Some(ins) = stack.get(self.stack_pointer) {
            return *ins;
        }
        Instruction::halt
    }

    fn combo_op(&self, op: u8) -> u64 {
        match op {
            0..=3 => op as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => {
                println!("invalid op ecncountered");
                0
            }
        }
    }

    fn flush(&mut self) {
        if self.out_buf.is_empty() {
            return;
        }
        if self.out_buf.len() > 1 {
            for val in &self.out_buf[..self.out_buf.len() - 1] {
                print!("{val},");
            }
        }
        print!("{}", self.out_buf[self.out_buf.len() - 1]);
        self.out_buf.clear();
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "reg A: {}", self.register_a)?;
        writeln!(f, "reg B: {}", self.register_b)?;
        writeln!(f, "reg C: {}", self.register_c)?;
        writeln!(f, "sp: {}", self.stack_pointer)?;
        writeln!(f, "buf: ")?;
        if self.out_buf.len() > 1 {
            for val in &self.out_buf[..self.out_buf.len() - 1] {
                write!(f, "{val},")?;
            }
        }
        if !self.out_buf.is_empty() {
            write!(f, "{}", self.out_buf[self.out_buf.len() - 1])?;
        } else {
            write!(f, "[]")?;
        }
        writeln!(f)?;
        Ok(())
    }
}

fn parse(data: PathBuf) -> Result<(Cpu, Stack)> {
    let f = fs::read_to_string(data)?;
    let mut cpu = Cpu::default();
    let f = f.lines().collect::<Vec<&str>>();
    let mut f = f.split(|line| line.is_empty());
    if let (Some(reg), Some(stack)) = (f.next(), f.next()) {
        if let (Some(a), Some(b), Some(c)) = (
            reg[0].strip_prefix("Register A: "),
            reg[1].strip_prefix("Register B: "),
            reg[2].strip_prefix("Register C: "),
        ) {
            cpu.register_a = a
                .parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse reg a".into()))?;
            cpu.register_b = b
                .parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse reg b".into()))?;
            cpu.register_c = c
                .parse::<u64>()
                .map_err(|_e| AOCError::ParseError("could not parse reg c".into()))?;
        }
        let mut s: Vec<Instruction> = Vec::new();
        if let Some(stack) = stack[0].strip_prefix("Program: ") {
            let stack = stack.split(',').collect::<Vec<&str>>();
            for pair in stack.chunks_exact(2) {
                let op = pair[1]
                    .parse::<u8>()
                    .map_err(|_e| AOCError::ParseError("could not parse operand".into()))?;
                let instruction = match pair[0] {
                    "0" => Instruction::adv(op),
                    "1" => Instruction::bxl(op),
                    "2" => Instruction::bst(op),
                    "3" => Instruction::jnz(op),
                    "4" => Instruction::bxc(op),
                    "5" => Instruction::out(op),
                    "6" => Instruction::bdv(op),
                    "7" => Instruction::cdv(op),
                    _ => return Err(AOCError::ParseError("could not parse instruction".into())),
                };
                s.push(instruction);
            }
        }
        return Ok((cpu, s));
    }
    Err(AOCError::ParseError("could not parse input".into()))
}
