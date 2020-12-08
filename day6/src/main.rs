use itertools::Itertools;
use std::{
    io::{self, Read, BufRead, BufReader},
};

fn count_yes_questions(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|l| l.chars().filter(|x| *x != '\n').sorted().dedup().count())
        .sum()
}

fn count_all_yes_questions(input: &str) -> usize {
   input
       .split("\n\n")
       .map(|l| {
           let group_size = l.lines().count();
           l.chars()
               .filter(|x| *x != '\n')
               .sorted()
               .map(|x| (x, 1))
               .coalesce(|(c1, f1), (c2, f2)| {
                   if c1 == c2 {
                       Ok((c1, f1 + f2))
                   } else {
                       Err(((c1, f1), (c2, f2)))
                   }
               }).filter(move |c| c.1 == group_size).count()
       }).sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut input_file = std::fs::File::open("input.txt")?;
    input_file.read_to_string(&mut input)?;

    println!("count: {}", count_yes_questions(&input));
    println!("all yes: {}", count_all_yes_questions(&input));
    Ok(())
}

#[test]
fn test0() {
    let example = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
    assert_eq!(count_yes_questions(&example), 11);
    assert_eq!(count_all_yes_questions(&example), 6);
}
