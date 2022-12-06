use std::fs;

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> String {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace(['\r', '\n'], "") // Strip all carriage returns (found on WSL)
}

fn index_of(c: &u8) -> usize {
    (c - b'a') as usize
}

fn find_marker(filepath: &str, window_size: usize) -> usize {
    let contents = read_and_parse(filepath);
    let bytes = contents.as_bytes();
    let window_size = window_size - 1;
    let mut histogram = [0; 26];
    for (_, c) in bytes.iter().enumerate().take(window_size) {
        histogram[index_of(c)] += 1;
    }
    for (index, c) in bytes.iter().enumerate().skip(window_size) {
        histogram[index_of(c)] += 1;
        if !histogram.iter().any(|x| *x > 1) {
            return index + 1;
        }
        histogram[index_of(&bytes[index - window_size])] -= 1;
    }
    0
}

fn main() {
    let part_01 = 4;
    let part_02 = 14;
    println!("Marker at: {}", find_marker("test_input_0.txt", part_01));
    println!("Marker at: {}", find_marker("test_input_1.txt", part_01));
    println!("Marker at: {}", find_marker("test_input_2.txt", part_01));
    println!("Marker at: {}", find_marker("test_input_3.txt", part_01));
    println!("Marker at: {}", find_marker("test_input_4.txt", part_01));
    println!("Market at: {}", find_marker("input.txt", part_01));
    println!("=========================");
    println!("Marker at: {}", find_marker("test_input_0.txt", part_02));
    println!("Marker at: {}", find_marker("test_input_1.txt", part_02));
    println!("Marker at: {}", find_marker("test_input_2.txt", part_02));
    println!("Marker at: {}", find_marker("test_input_3.txt", part_02));
    println!("Marker at: {}", find_marker("test_input_4.txt", part_02));
    println!("Market at: {}", find_marker("input.txt", part_02));
}
