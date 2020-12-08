use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
#[derive(Debug, Clone)]
enum Opcode {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}
#[derive(Debug, Clone)]
struct OpcodeParseErr;

impl FromStr for Opcode {
    type Err = OpcodeParseErr;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let mut l = l.split(' ').map(|x| x.to_string());
        let op = l.next().unwrap();
        let arg = l.next().unwrap().parse::<i32>().unwrap();
        let ins = match op.as_str() {
            "nop" => Ok(Opcode::Nop(arg)),
            "acc" => Ok(Opcode::Acc(arg)),
            "jmp" => Ok(Opcode::Jmp(arg)),
            _ => Err(OpcodeParseErr),
        };
        ins
    }
}

#[derive(Debug, Clone)]
struct MemoryCell {
    ins: Opcode,
    count: usize,
}

#[derive(Debug, Clone)]
struct HandHeld {
    memory: Vec<MemoryCell>,
    acc: i32,
    ip: i32,
}

impl HandHeld {
    fn load_program<T: AsRef<Path>>(name: T) -> io::Result<Self> {
        let memory = BufReader::new(std::fs::File::open(name)?)
            .lines()
            .map(|l| {
                let ins = l.unwrap().parse().unwrap();
                MemoryCell { ins, count: 0 }
            })
            .collect::<Vec<MemoryCell>>();

        Ok(HandHeld {
            memory,
            acc: 0,
            ip: 0,
        })
    }

    fn run_until_twice(&mut self) -> bool {
        while self.ip < self.memory.len() as i32 {
            if self.memory[self.ip as usize].count > 0 {
                return true;
            } else {
                self.memory[self.ip as usize].count += 1;
            }
            self.execute_instruction();
        }
        false
    }

    fn execute_instruction(&mut self) {
        match self.memory[self.ip as usize].ins {
            Opcode::Nop(_) => {}
            Opcode::Acc(a) => {
                self.acc += a;
            }
            Opcode::Jmp(ra) => {
                self.ip += ra;
                return;
            }
        }
        self.ip += 1;
    }

    fn run(&mut self) -> i32 {
        while self.ip < self.memory.len() as i32 {
            self.execute_instruction();
        }
        self.acc
    }

    fn patch_program(&mut self) {
        for ptr in 0..self.memory.len() {
            let old_ins = self.memory[ptr].ins.clone();
            match self.memory[ptr].ins {
                Opcode::Jmp(x) => self.memory[ptr].ins = Opcode::Nop(x),
                Opcode::Nop(x) => self.memory[ptr].ins = Opcode::Jmp(x),
                _ => {}
            }
            if !self.run_until_twice() {
                break;
            } else {
                self.reset();
                self.memory[ptr].ins = old_ins.clone();
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
        for m in self.memory.iter_mut() {
            m.count = 0;
        }
    }
}
fn main() -> io::Result<()> {
    let mut hh = HandHeld::load_program(std::env::args().skip(1).next().unwrap())?;
    hh.run_until_twice();
    println!("acc: {}", hh.acc);
    hh.reset();
    hh.patch_program();
    println!("acc after patching: {}", hh.run());
    Ok(())
}
