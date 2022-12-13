use pathfinding::prelude::dijkstra;
use std::fs;

type Pos = (usize, usize);
type Grid = Vec<Vec<u8>>;

type CostFunction = fn(i32, i32) -> bool;
type GoalFunction = fn(&Pos, &Grid) -> bool;

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Grid {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

fn find_start(grid: &Grid, search: u8) -> Pos {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| (x, y, c))
                .collect::<Vec<_>>()
        })
        .filter(|(_, _, c)| *c == &search)
        .map(|(x, y, _)| (x, y))
        .next()
        .expect("Cannot find start position")
}

fn cost(value: u8) -> i32 {
    (match value {
        b'S' => b'a',
        b'E' => b'z',
        _ => value,
    } - b'a') as i32
}

fn neighbours(pos: &Pos, grid: &Grid, f: CostFunction) -> Vec<(Pos, usize)> {
    let i_len = grid[0].len() as isize;
    let j_len = grid.len() as isize;
    let (x, y) = pos;
    let c = cost(grid[*y][*x]);
    let pos = (pos.0 as isize, pos.1 as isize);
    let (x, y) = pos;
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .map(|(a, b)| (x + a as isize, y + b as isize))
        .filter(|&(i, j)| 0 <= i && i < i_len && 0 <= j && j < j_len)
        .map(|(i, j)| (i as usize, j as usize))
        .filter(|&(i, j)| f(cost(grid[j][i]), c))
        .map(|(i, j)| ((i, j), 1))
        .collect::<Vec<_>>()
}

fn solve(filepath: &str, start: u8, f: CostFunction, g: GoalFunction) -> usize {
    let grid = read_and_parse(filepath);
    let start = find_start(&grid, start);
    let (_path, len) = dijkstra(&start, |pos| neighbours(pos, &grid, f), |pos| g(pos, &grid))
        .expect("Cannot find path in grid");
    len
}

fn main() {
    // Dijkstra's algorithm is an algorithm to find the best route from pos A to pos B.
    // Rather than implement it, we can simply use an implementation from a Rust crate
    // to solve part 1.
    //
    // For part 2, finding the best route from any 'a' to 'E' is the same as finding
    // the best route from 'E to any 'a' - which is what Dijkstra's algorithm does.
    // So to solve it, simply run dijkstra with flipped arguments:
    // - start is 'E', rather than 'S'
    // - `|m, n| n - m <= 1` rather than `|n, m| n - m <= 1` as we are descending rather than ascending
    // - goal is to find 'a' or 'S', rather than 'E'
    println!(
        "Fewest steps (part 1): {}",
        solve(
            "test_input.txt",
            b'S',
            |n, m| n - m <= 1,
            |&(x, y), grid| grid[y][x] == b'E'
        )
    );
    println!(
        "Fewest steps (part 2): {}",
        solve(
            "test_input.txt",
            b'E',
            |m, n| n - m <= 1,
            |&(x, y), grid| [b'a', b'S'].contains(&grid[y][x])
        )
    );
    println!("=========================");
    println!(
        "Fewest steps (part 1): {}",
        solve(
            "input.txt",
            b'S',
            |n, m| n - m <= 1,
            |&(x, y), grid| grid[y][x] == b'E'
        )
    );
    println!(
        "Fewest steps (part 2): {}",
        solve(
            "input.txt",
            b'E',
            |m, n| n - m <= 1,
            |&(x, y), grid| [b'a', b'S'].contains(&grid[y][x])
        )
    );
}
