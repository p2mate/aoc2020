use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct Machine {
    addr_mask_set: u64,
    data_mask_set: u64,
    data_mask_clear: u64,
    floating_bits: Vec<usize>,
    memory: HashMap<u64, u64>,
}

fn str_to_mask<F>(mask_str: &str, filter_f: F) -> u64
where
    F: Fn(char) -> bool,
{
    mask_str
        .chars()
        .enumerate()
        .filter(|b| filter_f(b.1))
        .fold(0, |acc, b| acc | (1 << mask_str.len() - 1 - b.0))
}

impl Machine {
    fn write_memory(&mut self, addr: u64, val: u64) {
        let v = (val | self.data_mask_set) & (!self.data_mask_clear);
        self.memory.insert(addr, v);
    }

    fn write_memory_mask(&mut self, addr: u64, val: u64) {
        let a = addr | self.addr_mask_set;

        fn map_number_on_bit_pattern(n :u64, bits: &[usize]) -> (u64, u64) {
            let mut clear = 0;
            let mut set = 0;
            for i in 0.. bits.len() {
                match n & (1 << i) {
                    0 => clear |= 1 << bits[i],
                    _ => set |= 1 << bits[i],
                }
            }
            (clear, set)
        }

        for i in 0..(1u64 << self.floating_bits.len()) {
            let (c, s) = map_number_on_bit_pattern(i, &self.floating_bits);
            self.memory.insert((a | s) & (!c), val);
        }
    }

    fn set_addr_mask(&mut self, mask: &str) {
        self.addr_mask_set = str_to_mask(mask, |x| x == '1');
        self.floating_bits = mask
            .chars()
            .enumerate()
            .filter(|b| b.1 == 'X')
            .map(|b| mask.len() - 1 - b.0)
            .collect();
    }

    fn set_data_mask(&mut self, mask: &str) {
        self.data_mask_clear = str_to_mask(mask, |x| x == '0');
        self.data_mask_set = str_to_mask(mask, |x| x == '1');
    }
}

fn day1<T: AsRef<Path>>(name: T) -> io::Result<()> {
    let input = BufReader::new(std::fs::File::open(name)?)
        .lines()
        .map(|l| l.unwrap());
    let mut m = Machine {
        data_mask_clear: 0,
        data_mask_set: 0,
        addr_mask_set: 0,
        memory: HashMap::new(),
        floating_bits: Vec::new(),
    };
    for l in input {
        match &l[0..2] {
            "ma" => m.set_data_mask(&l[6..]),
            "me" => {
                let mut fields = l.split(']');
                let addr = fields.next().unwrap()[4..].parse::<u64>().unwrap();
                let value = fields.next().unwrap()[3..].parse::<u64>().unwrap();
                m.write_memory(addr, value);
            }
            _ => unimplemented!(),
        }
    }

    println!("{}", m.memory.iter().fold(0, |acc, m| acc + m.1));
    Ok(())
}

fn day2<T: AsRef<Path>>(name: T) -> io::Result<()> {
    let mut m = Machine {
        data_mask_clear: 0,
        data_mask_set: 0,
        addr_mask_set: 0,
        memory: HashMap::new(),
        floating_bits: Vec::new(),
    };
    let input = BufReader::new(std::fs::File::open(name)?)
        .lines()
        .map(|l| l.unwrap());

    for l in input {
        match &l[0..2] {
            "ma" => m.set_addr_mask(&l[6..]),
            "me" => {
                let mut fields = l.split(']');
                let addr = fields.next().unwrap()[4..].parse::<u64>().unwrap();
                let value = fields.next().unwrap()[3..].parse::<u64>().unwrap();
                m.write_memory_mask(addr, value);
            }
            _ => unimplemented!(),
        }
    }

    println!("{}", m.memory.iter().fold(0, |acc, m| acc + m.1));

    Ok(())
}

fn main() -> io::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();

    day1(&filename)?;
    day2(&filename)?;

    Ok(())
}
