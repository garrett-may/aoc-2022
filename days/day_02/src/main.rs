use std::fs;

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<(u8, u8)> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.as_bytes()) // Rust cannot index into a string; convert to bytes i.e. ASCII first
        .map(|line| (line[0], line[2]))
        .collect::<Vec<_>>()
}

// Given an outcome in the following format:
//
// 0 => Draw
// 1 => Win
// 2 => Lose
//
// transform it into the following score:
//
// Lose => 0
// Draw => 3
// Win  => 6
//
fn score(outcome: i32) -> i32 {
    ((outcome + 1) % 3) * 3
}

// Calculate the total score for an input and using an interpretation function
fn calculate_score(filepath: &str, f: fn(i32, i32) -> i32) -> i32 {
    // Note that we find the difference from '@' for 'A', 'B', 'C', as
    // '@' is the ASCII character before 'A'. This gives us:
    //
    // 'A' => 1
    // 'B' => 2
    // 'C' => 3
    //
    // Similarly, 'W' is the ASCII character before 'X', so finding the
    // difference from that gives us:
    //
    // 'X' => 1
    // 'Y' => 2
    // 'Z' => 3
    read_and_parse(filepath)
        .into_iter()
        .map(|(you, me)| f((you - b'@') as i32, (me - b'W') as i32))
        .sum()
}

fn main() {
    // Part 1 and Part 2 are actually very similar, with the only difference
    // being the interpretation functions used. However, even these two functions
    // are very similar. The idea is the following:
    //
    // 	(you, me) => outcome
    //
    // Your shape, along with my shape, will provide some kind of outcome.
    // This turns out to be mathematical:
    //
    // 	you + outcome = me     [in modulus 3]
    //
    // i.e. if you find the difference between your shape and my shape, and then
    // you add/subtract 3 to the difference until it is within the bounds [0, 3)
    // (i.e. modulus 3; [0, 1, 2]), then you will get the outcome.
    //
    // There are a few caveats:
    // - you must do `+ 3` in part_01 in order to take the *modulus*, rather
    //   than the *remainder* (which is what `% 3` does by itself)
    //
    // - you must do `+ 1` in part_02 in order to get the correct value of
    //   the shape (RPS should be 1,2,3 rather than 0,1,2)
    //
    // - you must do `- 2` in part_02 in order to get the correct value of
    //   the outcome (LDW should be -1,0,1 rather than 1,2,3)

    let part_01 = |you, me| me + score((me - you) % 3 + 3);
    let part_02 = |you, outcome| (you + outcome) % 3 + 1 + score(outcome - 2);

    println!("Score: {}", calculate_score("test_input.txt", part_01));
    println!("Score: {}", calculate_score("input.txt", part_01));
    println!("=========================");
    println!("Score: {}", calculate_score("test_input.txt", part_02));
    println!("Score: {}", calculate_score("input.txt", part_02));
}
