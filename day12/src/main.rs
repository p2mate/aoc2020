use std::fmt::{self, Display};
use std::io::{self, BufRead, BufReader};

struct Ship {
    pos: Position,
    heading: Position,
}
struct Position {
    x: i32,
    y: i32,
}

const NORTH: Position = Position { x: 0, y: 1 };
const EAST: Position = Position { x: 1, y: 0 };
const SOUTH: Position = Position { x: 0, y: -1 };
const WEST: Position = Position { x: -1, y: 0 };

impl Position {
    fn move_direction(&mut self, units: i32, vector: &Position) {
        self.x += units * vector.x;
        self.y += units * vector.y;
    }

    fn rotate_right(&mut self, units: usize) {
        let units = (units % 360) / 90;
        for _steps in 0..units {
            let r = (self.y, -self.x);
            self.x = r.0;
            self.y = r.1;
        }
    }

    fn rotate_left(&mut self, units: usize) {
        let units = (units % 360) / 90;
        for _steps in 0..units {
            let r = (-self.y, self.x);
            self.x = r.0;
            self.y = r.1;
        }
    }
}

impl Ship {
    fn forward(&mut self, units: i32) {
        self.pos.x += units * self.heading.x;
        self.pos.y += units * self.heading.y;
    }

    fn manhattan(&self) -> usize {
        (self.pos.x.abs() + self.pos.y.abs()) as usize
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ew = match self.x.signum() {
            -1 => "West",
            _ => "East",
        };

        let ns = match self.y.signum() {
            -1 => "South",
            _ => "North",
        };
        write!(f, "({} {}, {} {})", ew, self.x.abs(), ns, self.y.abs(),)
    }
}

fn main() -> io::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let buffer = BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();

    let mut ship = Ship {
        pos: Position { x: 0, y: 0 },
        heading: Position { x: 1, y: 0 },
    };
    for l in buffer.clone() {
        let cmd = l.split_at(1);
        let units = cmd.1.parse::<i32>().unwrap();
        match cmd.0 {
            "F" => ship.forward(units),
            "N" => ship.pos.move_direction(units, &NORTH),
            "E" => ship.pos.move_direction(units, &EAST),
            "S" => ship.pos.move_direction(units, &SOUTH),
            "W" => ship.pos.move_direction(units, &WEST),
            "R" => ship.heading.rotate_right(units as usize),
            "L" => ship.heading.rotate_left(units as usize),
            _ => unimplemented!(),
        }
    }
    println!("{} manhattan distance: {}", ship.pos, ship.manhattan());

    let mut ship = Ship {
        pos: Position { x: 0, y: 0 },
        heading: Position { x: 10, y: 1 },
    };
    for l in buffer {
        let cmd = l.split_at(1);
        let units = cmd.1.parse::<i32>().unwrap();
        match cmd.0 {
            "F" => ship.forward(units),
            "N" => ship.heading.move_direction(units, &NORTH),
            "E" => ship.heading.move_direction(units, &EAST),
            "S" => ship.heading.move_direction(units, &SOUTH),
            "W" => ship.heading.move_direction(units, &WEST),
            "R" => ship.heading.rotate_right(units as usize),
            "L" => ship.heading.rotate_left(units as usize),
            _ => unimplemented!(),
        }
    }
    println!("{} manhattan distance: {}", ship.pos, ship.manhattan());
    Ok(())
}
