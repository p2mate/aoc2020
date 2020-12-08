fn calculate_seat(input: &str) -> usize {
    input.chars().fold(0, |acc, c| (acc << 1) | if c == 'B' || c == 'R' { 1 } else { 0})
}

use std::io::{self, Read};

fn main() -> io::Result<()> {
    use bitvec::prelude::*;
    use bitvec::vec::BitVec;

    let mut input = String::new();
    let mut input_file = std::fs::File::open("input.txt")?;
    input_file.read_to_string(&mut input)?;

    let highest_seat = input
        .lines()
        .fold(0, |acc, l| std::cmp::max(acc, calculate_seat(l.as_ref())));
    println!("Highest seat number: {}", highest_seat);
    let mut seat_mapping: BitVec<Lsb0> = BitVec::with_capacity(highest_seat + 1);
    seat_mapping.resize(highest_seat + 1, false);
    for l in input.lines() {
        seat_mapping.set(calculate_seat(l.as_ref()), true);
    }
    println!(
        "Seat: {}",
        seat_mapping[0..]
            .windows(3)
            .enumerate()
            .filter(|(_, x)| *x == bits![1, 0, 1])
            .next()
            .unwrap()
            .0
            + 1
    );
    Ok(())
}

#[test]
fn test0() {
    assert_eq!(calculate_seat("BFFFBBFRRR"), 567);
    assert_eq!(calculate_seat("FFFBBBFRRR"), 119);
    assert_eq!(calculate_seat("BBFFBBFRLL"), 820);
}
