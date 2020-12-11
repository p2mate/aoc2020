use itertools::Itertools;
use std::fmt::{self, Display};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
enum MapCell {
    Free,
    Occupied,
    Floor,
}

impl MapCell {
    fn is_free(&self) -> bool {
        *self == MapCell::Free
    }

    fn is_occupied(&self) -> bool {
        *self == MapCell::Occupied
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapCell::Free => write!(f, "L"),
            MapCell::Occupied => write!(f, "#"),
            MapCell::Floor => write!(f, "."),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct SeatMap {
    layout: Vec<Vec<MapCell>>,
}

fn neighbours_2(seatmap: &SeatMap, x: usize, y: usize) -> Vec<MapCell> {
    fn collect_neighbours(
        seatmap: &SeatMap,
        y_dir: i32,
        x_dir: i32,
        x: usize,
        y: usize,
    ) -> MapCell {
        let rows = seatmap.layout.len() as i32;
        let cols = seatmap.layout[0].len() as i32;
        let mut x_run = x as i32;
        let mut y_run = y as i32;
        loop {
            x_run += x_dir;
            y_run += y_dir;
            if !(x_run < cols && x_run >= 0 && y_run < rows && y_run >= 0) {
                break;
            }
            if seatmap.layout[y_run as usize][x_run as usize] != MapCell::Floor {
                return seatmap.layout[y_run as usize][x_run as usize].clone();
            }
        }
        return MapCell::Free;
    }

    let mut result = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            result.push(collect_neighbours(seatmap, i, j, x, y));
        }
    }
    result
}

fn neighbours(seatmap: &SeatMap, x: usize, y: usize) -> Vec<MapCell> {
    let mut result = Vec::new();
    let rows = seatmap.layout.len() as i32;
    let cols = seatmap.layout[0].len() as i32;
    for i in -1..=1 {
        for j in -1..=1 {
            let n_x = x as i32 + j;
            let n_y = y as i32 + i;
            if i == 0 && j == 0 {
                continue;
            }
            if n_y >= 0 && n_x >= 0 && n_y < rows && n_x < cols {
                result.push(seatmap.layout[n_y as usize][n_x as usize].clone());
            } else {
                result.push(MapCell::Free);
            }
        }
    }
    result
}

impl SeatMap {
    fn from_file<T: AsRef<Path>>(name: T) -> io::Result<Self> {
        let layout = BufReader::new(std::fs::File::open(name)?)
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        'L' => MapCell::Free,
                        '.' => MapCell::Floor,
                        _ => unimplemented!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(SeatMap { layout })
    }

    fn occupy_seats<F>(&mut self, neighbour_f: F) -> SeatMap
    where
        F: Fn(&SeatMap, usize, usize) -> Vec<MapCell>,
    {
        let rows = self.layout.len();
        let mut result = self.clone();
        for y in 0..rows {
            let cols = self.layout[0].len();
            for x in 0..cols {
                let neigbours = neighbour_f(self, x, y);
                let c = self.layout[y][x].clone();
                if c.is_free() && neigbours.iter().filter(|c| c.is_occupied()).count() == 0 {
                    result.layout[y][x] = MapCell::Occupied;
                } else if c.is_occupied()
                    && neigbours.iter().filter(|c| c.is_occupied()).count() >= 5
                {
                    result.layout[y][x] = MapCell::Free;
                }
            }
        }
        result
    }

    fn occupied_seats(&self) -> usize {
        self.layout
            .iter()
            .map(|x| x.iter().filter(|x| x.is_occupied()).count())
            .sum::<usize>()
    }
}

fn main() -> io::Result<()> {
    let mut seatmap = SeatMap::from_file(std::env::args().skip(1).next().unwrap())?;
    loop {
        let new_seatmap = seatmap.occupy_seats(neighbours);
        if seatmap == new_seatmap {
            break;
        } else {
            seatmap = new_seatmap;
        }
    }
    println!("occupied seats: {}", seatmap.occupied_seats());

    loop {
        let new_seatmap = seatmap.occupy_seats(neighbours_2);
        if seatmap == new_seatmap {
            break;
        } else {
            seatmap = new_seatmap;
        }
    }
    println!("occupied seats: {}", seatmap.occupied_seats());

    Ok(())
}
