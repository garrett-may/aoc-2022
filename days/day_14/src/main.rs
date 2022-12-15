use std::fs;
use std::iter::Map;
use std::ops::RangeInclusive;

type Coord = [usize; 2];
type Path = Vec<Coord>;
type Scan = Vec<Vec<char>>;
type Details = [usize; 4];

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<Path> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| {
            line.split(" -> ")
                .map(|coords| coords.split_once(',').expect("Cannot split coords pair"))
                .map(|(x, y)| {
                    [
                        x.parse::<usize>().expect("Cannot parse x coordinate"),
                        y.parse::<usize>().expect("Cannot parse y coordinate"),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

// I personally find it absolutely atrocious that `Step` is not implemented
// for `usize`, making me unable to have a `RangeInclusive<usize>` i.e. `(start..=end)`
fn range_incl(start: usize, end: usize) -> Map<RangeInclusive<isize>, fn(isize) -> usize> {
    ((start as isize)..=(end as isize)).map(|x| x as usize)
}

fn draw_line(scan: &mut Scan, min_x: usize, min_y: usize, start: &Coord, end: &Coord) {
    let [x0, y0] = start;
    let [x1, y1] = end;
    let [x0, x1, y0, y1] = [x0.min(x1), x0.max(x1), y0.min(y1), y0.max(y1)];
    if x0 == x1 {
        for y in range_incl(*y0, *y1) {
            scan[y - min_y][*x0 - min_x] = '#';
        }
    } else if y0 == y1 {
        for x in range_incl(*x0, *x1) {
            scan[*y0 - min_y][x - min_x] = '#';
        }
    } else {
        // It's good to know none of the lines were diagonals
        panic!("Found line which is neither vertical nor horizontal, but some kind of diagonal");
    }
}

fn fall_and_rest(scan: &mut Scan, details: Details) -> bool {
    let [min_x, _, min_y, _] = details;
    let mut sand = [500 - min_x, 0 - min_y];
    while sand[1] - min_y < scan.len() - 1 {
        if let Some([x, y]) = [sand[0], sand[0] - 1, sand[0] + 1]
            .iter()
            .map(|&x| [x, sand[1] + 1])
            .find(|[x, y]| scan[*y][*x] == '.')
        {
            // Keep falling
            sand = [x, y];
        } else {
            // At rest
            scan[sand[1]][sand[0]] = 'o';
            return true;
        }
    }
    scan[sand[1]][sand[0]] = 'o';
    false
}

fn sand_at_rest(filepath: &str, f: fn(&mut Scan), g: fn(&Scan, &Coord) -> bool) -> usize {
    let paths = read_and_parse(filepath);

    // Give a bit of extra room in the grid, for e.g. sand falling
    // For Part 2 we increase the extra room to ensure the pyramid
    // of sand can fall and be at rest.
    let max_y = paths
        .iter()
        .flatten()
        .fold(usize::MIN, |acc, [_, y]| acc.max(*y));

    let [min_x, max_x] = [500 - max_y - 2, 500 + max_y + 2];
    let min_y = 0; // Defined by sand source
    let (width, height) = (max_x - min_x + 2, max_y - min_y + 3);
    let source = [500 - min_x, 0 - min_y];

    let mut scan = vec![vec!['.'; width]; height];
    paths.iter().for_each(|path| {
        path[..]
            .windows(2)
            .for_each(|window| draw_line(&mut scan, min_x, min_y, &window[0], &window[1]))
    });
    f(&mut scan);
    scan[source[1]][source[0]] = '+';

    let mut units = 0;
    // Note: check first to see if the source is blocked, before
    // simulating the sand and thus potentially blocking the source
    while g(&scan, &source) && fall_and_rest(&mut scan, [min_x, max_x, min_y, max_y]) {
        units += 1;
    }

    units
}

fn main() {
    // For Part 1, simply simulate a sand falling until it reaches a limit.
    //
    // For Part 2, add the floor on the bottom as an extra line. Then simulate
    // the sand as in Part 1 until the source is also blocked.
    //
    // I used a grid that was able to hold enough information about the cave.
    // This made sense as actually, even though the puzzle mentions the floor
    // is infinite, the sand that falls is not; it will always create this
    // pyramid shap (with holes), and thus the shape and size can be predetermined
    // before simulating the sand.
    //
    // An alternatively method to using a grid was to use a recursive function and
    // depth first search. That sounds like an interesting idea, but I didn't attempt
    // it. It may or may not be faster than the grid method.

    let add_floor = |scan: &mut Scan| {
        let len = scan.len();
        scan[len - 1].iter_mut().for_each(|x| *x = '#')
    };
    let is_source_blocked = |scan: &Scan, source: &Coord| scan[source[1]][source[0]] == '+';
    println!("{:?}", sand_at_rest("test_input.txt", |_| {}, |_, _| true));
    println!(
        "{:?}",
        sand_at_rest("test_input.txt", add_floor, is_source_blocked)
    );
    println!("=========================");
    println!("{:?}", sand_at_rest("input.txt", |_| {}, |_, _| true));
    println!(
        "{:?}",
        sand_at_rest("input.txt", add_floor, is_source_blocked)
    );
}
