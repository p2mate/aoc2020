use std::io::{self, BufRead, BufReader};

fn calc_nearest(time: i64, bus_id: i64) -> i64 {
    let m = time / bus_id;
    if m * bus_id < time {
        (m + 1) * bus_id - time
    } else {
        m * bus_id - time
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], moduli: &[i64]) -> i64 {
    let prod = moduli.iter().product::<i64>();
    residues
        .iter()
        .zip(moduli)
        .map(|z| {
            let p = prod / z.1;
            z.0 * mod_inv(p, *z.1).unwrap() * p
        })
        .sum::<i64>() % prod
}

fn calc_first_departure_in_order(bus_ids: &[(usize, i64)]) -> i64 {
    let residues = bus_ids
        .iter()
        .map(|id| id.1 - id.0 as i64)
        .collect::<Vec<_>>();
    let moduli = bus_ids.iter().map(|id| id.1 as i64).collect::<Vec<_>>();
    chinese_remainder(&residues, &moduli)
}

fn parse_bus_ids(input: &mut dyn Iterator<Item = String>) -> Vec<(usize, i64)> {
    input
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .map(|x| (x.0, x.1.to_string()))
        .filter(|x| x.1 != "x")
        .map(|x| (x.0, x.1.parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
}
fn main() -> io::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let mut input = BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|l| l.unwrap());
    let earliest_departure = input.next().unwrap().parse::<i64>().unwrap();
    let bus_ids = parse_bus_ids(&mut input);
    let earliest_bus = bus_ids
        .iter()
        .map(|x| (x.1, calc_nearest(earliest_departure, x.1)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    println!("{}", earliest_bus.0 * earliest_bus.1);
    println!("{}", calc_first_departure_in_order(bus_ids.as_slice()));
    Ok(())
}

#[test]
fn test0() {
    let test_input = [
        ("17,x,13,19", 3417),
        ("67,7,59,61", 754018),
        ("67,x,7,59,61", 779210),
        ("67,7,x,59,61", 1261476),
        ("1789,37,47,1889", 1202161486),
    ];

    for input in test_input.iter() {
        assert_eq!(
            calc_first_departure_in_order(&parse_bus_ids(
                &mut input.0.split(' ').map(|x| x.to_string())
            )),
            input.1
        );
    }
}
