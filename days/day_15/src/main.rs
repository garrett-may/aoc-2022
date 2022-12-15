use std::collections::HashSet;
use std::fs;

type Coord = (i32, i32);
type Span = [i32; 2];

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<[Coord; 2]> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.split_once(':').expect("Cannot split line by colon"))
        .map(|(left, right)| {
            (
                left.split_once(',')
                    .expect("Cannot split left side by comma"),
                right
                    .split_once(',')
                    .expect("Cannot split right side by comma"),
            )
        })
        .map(|((sx, sy), (bx, by))| {
            [
                (
                    sx.split_once('=')
                        .expect("Cannot split sensor X by =")
                        .1
                        .parse::<i32>()
                        .expect("Cannot parse sensor X as i32"),
                    sy.split_once('=')
                        .expect("Cannot split sensor Y by =")
                        .1
                        .parse::<i32>()
                        .expect("Cannot parse sensor Y as i32"),
                ),
                (
                    bx.split_once('=')
                        .expect("Cannot split beacon X by =")
                        .1
                        .parse::<i32>()
                        .expect("Cannot parse beacon X as i32"),
                    by.split_once('=')
                        .expect("Cannot split beacon Y by =")
                        .1
                        .parse::<i32>()
                        .expect("Cannot parse beacon Y as i32"),
                ),
            ]
        })
        .collect::<Vec<_>>()
}

fn distance(x0: i32, x1: i32) -> i32 {
    (x0 - x1).abs()
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i32 {
    distance(a.0, b.0) + distance(a.1, b.1)
}

fn span(sensor: &Coord, beacon: &Coord, y: i32) -> Option<Span> {
    let (sx, sy) = sensor;
    let d = manhattan_distance(sensor, beacon);
    let dx = d - distance(*sy, y);
    if dx >= 0 {
        Some([sx - dx, sx + dx])
    } else {
        None
    }
}

// Spans can overlap each other, which would affect the couting the
// positions that cannot contain a beacon. So we need to fix them.
fn handle_overlaps(mut overlapping_spans: Vec<Span>) -> Vec<Span> {
    overlapping_spans.sort_by(|[a0, _], [b0, _]| a0.cmp(b0));
    let mut spans = Vec::<Span>::new();
    let mut span = overlapping_spans[0];
    for overlapping_span in overlapping_spans.iter().skip(1) {
        if span[1] + 1 < overlapping_span[0] {
            spans.push(span);
            span = *overlapping_span;
        } else {
            span[1] = span[1].max(overlapping_span[1]);
        }
    }
    spans.push(span);
    spans
}

fn coords_without_beacon(filepath: &str, y: i32) -> i32 {
    let report = read_and_parse(filepath);
    let sensors_and_beacons = report.iter().flatten().collect::<HashSet<_>>();
    let spans = report
        .iter()
        .map(|[sensor, beacon]| span(sensor, beacon, y))
        .filter(|maybe_span| !maybe_span.is_none())
        .map(|maybe_span| maybe_span.expect("No span available"))
        .collect::<Vec<_>>();
    let spans = handle_overlaps(spans);
    spans.iter().map(|[x0, x1]| x1 - x0 + 1).sum::<i32>()
        - sensors_and_beacons
            .iter()
            .filter(|item| item.1 == y)
            .count() as i32
}

// Part 2: Idea #1
//
// Rather than search through 4m^2 coordinates, use Part 1 to find the
// X coordinate for us.
//
// We are guaranteed that there is only a single position that could
// be the distress beacon. So this means that most Y coordinates will
// have 1 or 0 spans; but there will be exactly one Y coordinate with
// exactly 2 spans, with a break in the middle. This break will be the
// distress beacon.
//
/*fn find_distress_beacon(filepath: &str, pair: (i32, i32)) -> usize {
    let report = read_and_parse(filepath);

    let (start, end) = pair;
    for y in start..end {
        let spans = report
            .iter()
            .map(|[sensor, beacon]| span(sensor, beacon, y))
            .filter(|maybe_span| !maybe_span.is_none())
            .map(|maybe_span| maybe_span.expect("No span available"))
            .collect::<Vec<_>>();
        if spans.is_empty() {
            continue;
        }
        let spans = handle_overlaps(spans);
        if spans.len() == 2 {
            let x = spans[0][1] + 1;
            return (x as usize) * 4000000 + (y as usize);
        }
    }
    0
}*/

fn search_borders(
    sensor: &Coord,
    beacon: &Coord,
    report: &[[Coord; 2]],
    pair: (i32, i32),
) -> Option<Coord> {
    let (start, end) = pair;
    let (sx, sy) = sensor;
    let d = manhattan_distance(sensor, beacon) + 1;
    for dxdy in 0..=d {
        for (x, y) in [
            (sx + dxdy, sy + d - dxdy),
            (sx - dxdy, sy + d - dxdy),
            (sx + dxdy, sy - d + dxdy),
            (sx - dxdy, sy - d + dxdy),
        ] {
            if !(start..=end).contains(&x) || !(start..=end).contains(&y) {
                continue;
            }
            if !report
                .iter()
                .any(|[s, b]| manhattan_distance(s, &(x, y)) <= manhattan_distance(s, b))
            {
                return Some((x, y));
            }
        }
    }
    None
}

// Part 2: Idea #2
//
// Rather than search through 4m^2 coordinates, search the borders of
// each sensor's range.
//
// We are guaranteed that there is only a single position that could
// be the distress beacon. As the distress beacon is outside all the
// sensors range, this means we can check the positions just outside
// the borders of each sensor's range to see if that position is the
// distress beacon.
//
fn find_distress_beacon_2(filepath: &str, pair: (i32, i32)) -> usize {
    let report = read_and_parse(filepath);
    let (x, y) = report
        .iter()
        .map(|[sensor, beacon]| search_borders(sensor, beacon, &report, pair))
        .find(|coord| !coord.is_none())
        .expect("No distress beacon")
        .expect("No distress beacon");
    (x as usize) * 4000000 + (y as usize)
}

fn main() {
    // Part 1 was reasonably simple. We cannot realistically use a
    // HashSet for the positions that cannot contain a beacon, due to
    // space requirements. So instead we take the spans for each sensor
    // and then union them together.
    //
    // Part 2 is tricky. There were two ideas (described above).
    // - Idea #1 makes use of most of the same code as Part 1
    // - Idea #2 is faster than Idea #1
    //
    // Both ideas are written here for the reader's benefit.

    println!("{:?}", coords_without_beacon("test_input.txt", 10));
    println!("{:?}", coords_without_beacon("input.txt", 2000000));
    println!("=========================");

    //println!("{:?}", find_distress_beacon("test_input.txt", (0, 20)));
    //println!("{:?}", find_distress_beacon("input.txt", (0, 4000000)));

    println!("{:?}", find_distress_beacon_2("test_input.txt", (0, 20)));
    println!("{:?}", find_distress_beacon_2("input.txt", (0, 4000000)));
}
