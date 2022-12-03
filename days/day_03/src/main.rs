#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<std::string::String> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}

fn priority(c: u8) -> usize {
    if (b'a'..=b'z').contains(&c) {
        ((c - b'a') as usize) + 1
    } else if (b'A'..=b'Z').contains(&c) {
        ((c - b'A') as usize) + 27
    } else {
        0
    }
}

// `&a & &b` means to find the intersection between the HashSet `a`
// and the HashSet `b`. Returns another HashSet.

fn part_01(filepath: &str) -> usize {
    read_and_parse(filepath)
        .iter()
        .map(|line| {
            let (bytes, m) = (line.as_bytes(), line.len() / 2);
            let a = HashSet::<u8>::from_iter(bytes[..m].iter().cloned());
            let b = HashSet::<u8>::from_iter(bytes[m..].iter().cloned());
            (&a & &b)
                .drain()
                .next()
                .expect("No elements in intersection")
        })
        .map(priority)
        .sum()
}

fn part_02(filepath: &str) -> usize {
    read_and_parse(filepath)
        .iter()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let a = HashSet::<u8>::from_iter(a.as_bytes().iter().cloned());
            let b = HashSet::<u8>::from_iter(b.as_bytes().iter().cloned());
            let c = HashSet::<u8>::from_iter(c.as_bytes().iter().cloned());
            (&(&a & &b) & &c)
                .drain()
                .next()
                .expect("No elements in intersection")
        })
        .map(priority)
        .sum()
}

fn main() {
    println!("Total: {}", part_01("test_input.txt"));
    println!("Total: {}", part_01("input.txt"));
    println!("=========================");
    println!("Total: {}", part_02("test_input.txt"));
    println!("Total: {}", part_02("input.txt"));
}
