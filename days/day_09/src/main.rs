use std::collections::HashSet;
use std::fs;

type Point = (i32, i32);
type Motion = (u8, usize);

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<Motion> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| {
            let (a, b) = line.split_once(' ').expect("Cannot split line");
            (
                a.as_bytes()[0],
                b.parse::<usize>().expect("Cannot parse steps as usize"),
            )
        })
        .collect::<Vec<_>>()
}

fn drag(x: i32, y: i32) -> i32 {
    y + x.cmp(&y) as i32
}

fn drag_rope<const KNOTS: usize>(direction: u8, knots: &mut [Point; KNOTS]) -> Point {
    match direction {
        b'R' => knots[0].0 += 1,
        b'L' => knots[0].0 -= 1,
        b'U' => knots[0].1 += 1,
        b'D' => knots[0].1 -= 1,
        _ => {}
    }
    for i in 0..KNOTS - 1 {
        let ((hx, hy), (tx, ty)) = (knots[i], knots[i + 1]);
        if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
            knots[i + 1] = (drag(hx, tx), drag(hy, ty));
        }
    }
    knots[KNOTS - 1]
}

fn count_tail_positions<const KNOTS: usize>(filepath: &str) -> usize {
    let mut knots = [(0, 0); KNOTS];
    read_and_parse(filepath)
        .into_iter()
        .flat_map(|(direction, steps)| vec![direction; steps])
        .map(|direction| drag_rope(direction, &mut knots))
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    println!(
        "No. of positions: {}",
        count_tail_positions::<2>("test_input_0.txt")
    );
    println!(
        "No. of positions: {}",
        count_tail_positions::<2>("test_input_1.txt")
    );
    println!(
        "No. of positions: {}",
        count_tail_positions::<2>("input.txt")
    );
    println!("=========================");
    println!(
        "No. of positions: {}",
        count_tail_positions::<10>("test_input_0.txt")
    );
    println!(
        "No. of positions: {}",
        count_tail_positions::<10>("test_input_1.txt")
    );
    println!(
        "No. of positions: {}",
        count_tail_positions::<10>("input.txt")
    );
}
