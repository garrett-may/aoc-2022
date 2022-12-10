use std::fs;

// Note: we can transform: `"noop"` and `"addx X"` into `["noop"]` and `["addx", "X"]`;
// this maps each item to as many cycles it takes to do the initial item. We can then
// `enumerate()` to retrieve the cycle, and then discard the items (except for "X" which
// we parse into an integer).

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<String> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .flat_map(|line| line.split(' '))
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
}

fn cycles(filepath: &str) -> Vec<(i32, i32)> {
    let mut x = 1;
    read_and_parse(filepath)
        .into_iter()
        .enumerate()
        .map(|(index, item)| (index as i32 + 1, item.parse::<i32>().unwrap_or(0)))
        .map(&mut |(cycle_no, value)| {
            let r = (cycle_no, x);
            x += value;
            r
        }) // Store result before adding, as we need the value *during*, not *after*.
        .collect::<Vec<_>>()
}

fn sum_of_signal_strengths(filepath: &str) -> i32 {
    cycles(filepath)
        .into_iter()
        .filter(|(cycle_no, _)| (cycle_no + 20) % 40 == 0)
        .map(|(cycle_no, x)| cycle_no * x)
        .sum()
}

fn render_image(filepath: &str) -> String {
    cycles(filepath)
        .into_iter()
        .map(|(cycle_no, x)| {
            (
                cycle_no,
                if (x - 1..=x + 1).contains(&((cycle_no - 1) % 40)) {
                    "#"
                } else {
                    "."
                },
            )
        })
        .map(|(cycle_no, c)| {
            if cycle_no % 40 == 0 {
                format!("{c}\r\n")
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}

fn main() {
    println!("Sum: {}", sum_of_signal_strengths("test_input.txt"));
    print!("{}", render_image("test_input.txt"));
    println!("=========================");
    println!("Sum: {}", sum_of_signal_strengths("input.txt"));
    print!("{}", render_image("input.txt"));
}
