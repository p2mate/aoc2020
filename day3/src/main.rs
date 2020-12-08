use std::io;
use std::io::Read;

fn find_slopes(grid: &[&str], x_inc: usize, y_inc: usize) -> usize {
    let mut posx = 0;
    let mut posy = 0;
    let mut trees = 0;
    while posy < grid.len() {
        if grid[posy].chars().nth(posx).unwrap() == '#' {
            trees += 1;
        }        
        posx = (posx + x_inc) % grid[0].len();
        posy += y_inc;
    }
    trees
}
fn main() -> io::Result<()> {  
    let mut input = std::fs::File::open("input.txt")?;
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let grid = buffer.lines().collect::<Vec<_>>();

    println!("number of trees: {}", find_slopes(&grid, 3, 1));
    println!("number of trees multiplied: {}", [ (1, 1), (3, 1), (5,1), (7,1), (1,2) ].iter().fold(1, |acc, x| acc * find_slopes(&grid, x.0, x.1)));
    Ok(())
}

#[test]

fn test() {
    let grid = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    assert_eq!(find_slopes(&grid, 3, 1), 7);
    assert_eq!([ (1, 1), (3, 1), (5,1), (7,1), (1,2) ].iter().map(|x| find_slopes(&grid, x.0, x.1) as u32).product::<u32>(), 336);
}