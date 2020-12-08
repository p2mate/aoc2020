#[derive(Debug, Clone)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

use std::io;

fn check_one(input: &[PasswordPolicy]) -> usize {
    input
        .iter()
        .filter(|x| {
            let count = x.password.match_indices(x.letter).count();
            count >= x.min && count <= x.max
        })
        .count()
}

fn check_two(input: &[PasswordPolicy]) -> usize {
    input
        .iter()
        .filter(|x| {
            let first = x.password[x.min - 1..x.min].chars().next().unwrap() == x.letter;
            let second = x.password[x.max - 1..x.max].chars().next().unwrap() == x.letter;
            (first || second) && !(first && second)
        })
        .count()
}

fn main() -> io::Result<()> {
    let example = vec![
        PasswordPolicy {
            min: 1,
            max: 3,
            letter: 'a',
            password: String::from("abcde"),
        },
        PasswordPolicy {
            min: 1,
            max: 3,
            letter: 'b',
            password: String::from("cdefg"),
        },
        PasswordPolicy {
            min: 2,
            max: 9,
            letter: 'c',
            password: String::from("ccccccccc"),
        },
    ];
    println!("{}", check_one(&example));

    let mut input = Vec::new();

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(x) if x > 0 => {
                let min_sign_index = buffer.find('-').unwrap();
                let min = buffer[0..min_sign_index].parse().unwrap();
                let buffer_next = &buffer[min_sign_index + 1..];
                let space_index = buffer_next.find(' ').unwrap();
                let max = buffer_next[..space_index].parse().unwrap();
                let letter = buffer_next[space_index + 1..space_index + 2]
                    .chars()
                    .next()
                    .unwrap();
                let password = buffer_next[space_index + 4..].trim_end().to_string();
                input.push(PasswordPolicy {
                    min,
                    max,
                    letter,
                    password,
                });
            }
            _ => break,
        }
    }

    println!("{}", check_one(&input));

    println!("{}", check_two(&example));

    println!("{}", check_two(&input));

    Ok(())
}
