use std::fs;

type Section = (u32, u32);

type Pair = (Section, Section);

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<Pair> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.split_once(',').expect("Cannot split line into pair"))
        .map(|(first, second)| {
            (
                first
                    .split_once('-')
                    .expect("Cannot split first section into pair"),
                second
                    .split_once('-')
                    .expect("Cannot split second section into pair"),
            )
        })
        .map(|((x0, y0), (x1, y1))| {
            (
                (
                    x0.parse::<u32>()
                        .expect("Cannot parse first value of first section"),
                    y0.parse::<u32>()
                        .expect("Cannot parse second value of first section"),
                ),
                (
                    x1.parse::<u32>()
                        .expect("Cannot parse first value of second section"),
                    y1.parse::<u32>()
                        .expect("Cannot parse second value of second section"),
                ),
            )
        })
        .collect::<Vec<_>>()
}

fn count_overlaps(filepath: &str, f: fn(Pair) -> bool) -> usize {
    read_and_parse(filepath)
        .into_iter()
        .filter(|((x0, y0), (x1, y1))| f(((*x0, *y0), (*x1, *y1))))
        .count()
}

fn main() {
    // Note that for any section `(x, y)` that `x <= y` always holds true

    // Find overlaps where one section completely contains the other section
    // ... which occurs when one section has its boundaries fully inside the others
    let part_01 = |((x0, y0), (x1, y1))| (x0 <= x1 && y1 <= y0) || (x1 <= x0 && y0 <= y1);
    // Find overlaps where any part of the pair of sections overlap
    // ... which is the opposite of finding pairs which *do not* overlap
    let part_02 = |((x0, y0), (x1, y1))| !(y0 < x1 || y1 < x0);

    println!("Total: {}", count_overlaps("test_input.txt", part_01));
    println!("Total: {}", count_overlaps("input.txt", part_01));
    println!("=========================");
    println!("Total: {}", count_overlaps("test_input.txt", part_02));
    println!("Total: {}", count_overlaps("input.txt", part_02));
}
