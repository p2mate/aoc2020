use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let mut spoken = HashMap::new();
    let mut turn = 1;
    let mut last = 0;

    for n in BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|l| l.unwrap())
        .fold(String::new(), |acc, l| acc + &l)
        .split(',')
    {
        last = n.parse::<u32>().unwrap();
        spoken.insert(last, vec![turn]);
        turn += 1;
    }

    while turn <= 30000000 {
        let speak;
        match spoken.get(&last) {
            Some(v) => {
                match v.len() {
                    1 => {
                        speak = 0;
                    }
                    2 => {
                        speak = v[1] - v[0];
                    }
                    _ => unimplemented!(),
                }
            }
            _ => speak = 0,
        }
        match spoken.get(&speak) {
            Some(v) => {
                let mut v2 = v.clone();
                match v.len() {
                    1 => v2.push(turn),
                    2 => {
                        v2[0] = v2[1];
                        v2[1] = turn;
                    }
                    _ => unimplemented!(),
                }
                spoken.insert(speak, v2);
            }
            None => {
                spoken.insert(speak, vec![turn]);
            }
        }
        last = speak;
        turn += 1;
    }
    println!("{}", last);
    Ok(())
}
