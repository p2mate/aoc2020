use itertools::Itertools;
use std::str::FromStr;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader, Read},
};

#[derive(Debug, Clone)]
struct Range {
    r_begin: usize,
    r_end: usize,
}
#[derive(Debug, Clone)]
struct RangeParseErr;

impl From<std::num::ParseIntError> for RangeParseErr {
    fn from(_: std::num::ParseIntError) -> Self {
        RangeParseErr
    }
}

impl FromStr for Range {
    type Err = RangeParseErr;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let mut input_iter = l.split('-');
        if let Some(b) = input_iter.next() {
            if let Some(e) = input_iter.next() {
                Ok(Range {
                    r_begin: b.parse::<usize>()?,
                    r_end: e.parse::<usize>()?,
                })
            } else {
                Err(RangeParseErr)
            }
        } else {
            Err(RangeParseErr)
        }
    }
}
#[derive(Debug, Clone)]
struct ValidRanges {
    ranges: Vec<(String, Vec<Range>)>,
}

impl ValidRanges {
    fn validate_ticket(&self, ticket: &str) -> Vec<usize> {
        let mut errors = Vec::new();
        for field in ticket.split(',') {
            let v = field.parse::<usize>().unwrap();
            if self
                .ranges
                .iter()
                .map(|r| r.1.iter().filter(|x| v >= x.r_begin && v <= x.r_end))
                .flatten()
                .count()
                == 0
            {
                errors.push(v);
            }
        }
        errors
    }
}

fn check_tickets_in_range(range: &Vec<Range>, tickets: &Vec<usize>) -> bool {
    tickets
        .iter()
        .filter(|t| {
            (t >= &&range[0].r_begin && t <= &&range[0].r_end)
                || (t >= &&range[1].r_begin && t <= &&range[1].r_end)
        })
        .count() == tickets.len()
}

fn main() -> io::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let mut input = String::new();
    std::fs::File::open(filename)?.read_to_string(&mut input)?;
    let mut input_iter = input.split("\n\n");
    let ranges = input_iter.next().unwrap();
    let my_ticket = input_iter.next().unwrap(); // .split("\n\n").next().unwrap();
    let nearby_tickets = input_iter.next().unwrap(); //.split("\n\n").skip(1).next().unwrap();

    let mut valid_ranges = ValidRanges { ranges: Vec::new() };
    for (i, l) in ranges.lines().enumerate() {
        let range_name = l.split(':').next().unwrap();
        let range_fields = l.split(':').skip(1).next().unwrap().trim_start();
        let mut ranges = Vec::new();
        for r in range_fields.split(' ').step_by(2) {
            ranges.push(r.parse().unwrap());
        }
        valid_ranges.ranges.push((range_name.to_string(), ranges));
    }

    println!(
        "nearby tickets: {}",
        nearby_tickets
            .lines()
            .skip(1)
            .map(|t| valid_ranges.validate_ticket(t))
            .flatten()
            .sum::<usize>()
    );

    let mut result = 1;
    let my_ticket = my_ticket
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let nearby_tickets_table = (0..my_ticket.len())
        .map(|x| {
            nearby_tickets
                .lines()
                .skip(1)
                .filter(|t| valid_ranges.validate_ticket(t).is_empty())
                .map(|y| {
                    y.split(',')
                        .skip(x)
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let possible_fields = valid_ranges
        .ranges
        .iter()
        .map(|r| {
            (
                r.0.clone(),
                (0..my_ticket.len())
                    .filter(|i| check_tickets_in_range(&r.1, &nearby_tickets_table[*i]))
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut temp_rules = possible_fields
        .iter()
        .sorted_by(|a, b| a.1.len().cmp(&b.1.len()))
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    loop {
        let r = temp_rules[0].clone();
        assert_eq!(r.1.len(), 1);
        let field = r.1.iter().next().unwrap();
        if r.0.len() >= 9 && &r.0[0..9] == "departure" {
            result *= my_ticket[*field];
        }

        temp_rules = temp_rules[1..]
            .iter()
            .map(|x| {
                let mut t = x.1.clone();
                t.retain(|e| e != field);
                (x.0.clone(), t.clone())
            })
            .collect::<Vec<_>>();
        if temp_rules.len() == 0 {
            break;
        }
    }



    println!("result: {}", result);
    Ok(())
}
