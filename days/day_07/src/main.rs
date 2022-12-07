use std::fs;

enum IO {
    CD(bool),
    File(u32),
}

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<IO> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .filter(|line| !matches!(&line[0..3], "$ l" | "dir")) // We do not need "$ ls" or "dir X" for the puzzle
        .map(|line| match line.as_bytes()[0] {
            b'$' => IO::CD(&line[5..] == ".."),
            _ => IO::File(
                line.split_once(' ')
                    .expect("Cannot split file")
                    .0
                    .trim()
                    .parse::<u32>()
                    .expect("Cannot parse file size as u32"),
            ),
        })
        .collect::<Vec<_>>()
}

// Traverse through the iterator as though it were a directory-file tree
fn traverse<F>(iter: &mut std::slice::Iter<'_, IO>, f: &mut F) -> u32
where
    F: FnMut(u32),
{
    match iter.next() {
        // "$ cd X" where "X" is *not* ".."
        Some(IO::CD(false)) => {
            // Traverse through the current working directory until we stop
            let sum = traverse(iter, f);
            f(sum);
            sum + traverse(iter, f)
        }
        // "123 abc.txt"
        Some(IO::File(size)) => size + traverse(iter, f),
        // Everything else stops traversing through the current working directory
        _ => 0,
    }
}

fn part_01_and_part_02(filepath: &str) {
    let lines = read_and_parse(filepath);
    let mut part_01 = 0;
    let used_space = traverse(&mut lines.iter(), &mut |sum| {
        if sum <= 100000 {
            part_01 += sum;
        }
    });
    let needed_space = used_space - 40000000;
    let mut part_02 = used_space;
    traverse(&mut lines.iter(), &mut |sum| {
        if needed_space <= sum && sum < part_02 {
            part_02 = sum;
        }
    });
    println!("Part 1: {part_01}");
    println!("Part 2: {part_02}");
}

fn main() {
    part_01_and_part_02("test_input.txt");
    println!("=========================");
    part_01_and_part_02("input.txt");
}
