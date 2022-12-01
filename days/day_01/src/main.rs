use std::fs;

// Read in a file as a string, and then handle it so that
// groups of lines, separated by newlines, are collected
// together within a vector of vectors
//
// e.g.
//
// "100\r\n200\r\n\r\n300\r\n"
//
// becomes:
//
// [["100", "200], ["300"]]
//
fn read_file(filepath: &str) -> Vec<Vec<std::string::String>> {
    fs::read_to_string(["../res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split("\n\n")
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| {
            line.split('\n')
                .filter(|line| line != &"") // Remove extraneous empty lines
                .map(|line| line.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

// Find greatest calories any elf holds, and then find the
// sum of the 3 greatest calories elves hold
fn find_calories(filepath: &str) {
    let lines = read_file(filepath);

    // Parse each string as an i32
    let values = lines
        .into_iter()
        .map(|lines| {
            lines
                .into_iter()
                .map(|line| line.parse::<i32>().expect("Unable to parse string as i32"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find the sum of calories each elf holds
    let mut sums = values
        .into_iter()
        .map(|values| values.into_iter().sum())
        .collect::<Vec<i32>>();

    // Finding the maximum is better for part 1, but
    // sorting is better if doing both part 1 and part 2
    sums.sort_by(|a, b| b.cmp(a));
    let part_01 = sums[0];
    let part_02 = sums.iter().take(3).sum::<i32>();

    println!("Answer to part 1: {}", part_01);
    println!("Answer to part 2: {}", part_02);
}

fn main() {
    find_calories("test_input.txt");
    println!("=========================");
    find_calories("input.txt");
}
