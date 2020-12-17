use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct World {
    map: Vec<Vec<Vec<Vec<bool>>>>,
}

impl World {
    fn from_file<T: AsRef<Path>>(name: T) -> io::Result<Self> {
        let layout = BufReader::new(std::fs::File::open(name)?)
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unimplemented!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(World {
            map: vec![vec![layout]],
        })
    }

    fn in_map_t(&self, n: i32) -> bool {
        !(n < 0 || n >= self.map.len() as i32)
    }


    fn in_map_z(&self, n: i32) -> bool {
        !(n < 0 || n >= self.map[0].len() as i32)
    }

    fn in_map_xy(&self, n: i32) -> bool {
        !(n < 0 || n >= self.map[0][0].len() as i32)
    }
    fn neighbours(&self, x: usize, y: usize, z: usize, t: usize) -> Vec<bool> {
        let mut result = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        if i == 0 && j == 0 && k == 0 && l == 0 {
                            continue;
                        }
                        let x_n = x as i32 + l;
                        let y_n = y as i32 + k;
                        let z_n = z as i32 + j;
                        let t_n = t as i32 + i;
                        if self.in_map_xy(x_n)
                            && self.in_map_xy(y_n)
                            && self.in_map_z(z_n)
                            && self.in_map_t(t_n)
                        {
                            //   dbg!(self.map[z_n as usize][y_n as usize][x_n as usize]);
                            result.push(
                                self.map[t_n as usize][z_n as usize][y_n as usize][x_n as usize],
                            );
                        } else {
                            result.push(false);
                        }
                    }
                }
            }
        }
        println!("{} {} {} {} {:?} {}", x, y, z, t, result, result.len());
        result
    }

    fn step(&mut self) {
        let mut empty_plane = Vec::new();
        for y in 0..self.map[0][0].len() {
            empty_plane.push(Vec::new());
            for x in 0..self.map[0][0][0].len() {
                empty_plane[y].push(false);
            }
        }
        for t in 0..self.map.len() {
            self.map[t].insert(0, empty_plane.clone());
            self.map[t].push(empty_plane.clone());
            for z in 0..self.map[t].len() {
                let mut empty = Vec::new();
                empty.resize(self.map[t][z][0].len(), false);
                self.map[t][z].insert(0, empty.clone());
                self.map[t][z].push(empty);
                for y in 0..self.map[t][z].len() {
                    self.map[t][z][y].insert(0, false);
                    self.map[t][z][y].push(false);
                }
            }
        }

        let mut empty_cube = Vec::new();
        for z in 0..self.map[0].len() {
            empty_cube.push(Vec::new());
            for y in 0..self.map[0][0].len() {
                empty_cube[z].push(Vec::new());
                for x in 0..self.map[0][0][0].len() {
                    empty_cube[z][y].push(false);
                }
            }
        }

        self.map.insert(0, empty_cube.clone());
        self.map.push(empty_cube.clone());
        let mut new_map = self.map.clone();

        println!("{} {} {} {}", self.map.len(), self.map[0].len(), self.map[0][0].len(), self.map[0][0][0].len());

       dbg!(&self.map);
        for t in 0..self.map.len() {
            for z in 0..self.map[t].len() {
                for y in 0..self.map[t][z].len() {
                    for x in 0..self.map[t][z][y].len() {
                        let active_neighbours =
                            self.neighbours(x, y, z, t).iter().filter(|n| **n).count();
                        println!("{} {} {} {}: {}", x, y, z, t, active_neighbours);
                        if self.map[t][z][y][x] {
                            new_map[t][z][y][x] = active_neighbours == 2 || active_neighbours == 3;
                        } else {
                            new_map[t][z][y][x] = active_neighbours == 3
                        }
                    }
                }
            }
        }

        self.map = new_map;
    }
}

fn main() -> io::Result<()> {
    let mut world = World::from_file(std::env::args().skip(1).next().unwrap())?;
    dbg!(&world);
    for i in 0..6 {
        world.step();
        dbg!(&world);
    }
    let mut result = 0;
    for t in 0..world.map.len() {
        for z in 0..world.map[t].len() {
            for y in 0..world.map[t][z].len() {
                for x in 0..world.map[t][z][y].len() {
                    result += if world.map[t][z][y][x] { 1 } else { 0 };
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}
