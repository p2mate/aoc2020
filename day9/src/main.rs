use itertools::Itertools;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_input<T: AsRef<Path>>(name: T) -> io::Result<Vec<String>> {
    BufReader::new(std::fs::File::open(name)?).lines().collect()
}

fn main() -> io::Result<()> {
    let name = std::env::args().skip(1).next().unwrap();
    let numbers = read_input(name)?
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let odd_ball = numbers
        .windows(26)
        .filter(|n| {
            n[0..n.len() - 1]
                .iter()
                .combinations(2)
                .filter(|x| x[0] + x[1] == *n.last().unwrap())
                .next()
                .is_none()
        })
        .next()
        .unwrap()
        .last()
        .unwrap();
    println!("Odd ball: {}", odd_ball);
    println!(
        "Weakness: {}",
        (2..numbers.len())
            .into_iter()
            .filter_map(|i| numbers
                .windows(i)
                .filter(|x| x.iter().sum::<u64>() == *odd_ball)
                .next())
            .map(|x| x.iter().min().unwrap() + x.iter().max().unwrap())
            .next()
            .unwrap()
    );
    Ok(())
}
