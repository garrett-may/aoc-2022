use std::fs;

type Stack = Vec<u8>;

type Procedure = (usize, usize, usize);

fn parse_stacks(s: &str) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = Vec::new();
    s.split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .rev() // Start from the end i.e. the start of the stack, so that we `.push()` items to the back of the stack
        .for_each(|line| {
            line.as_bytes()
                .chunks(4)
                .enumerate()
                .for_each(|(index, bytes)| {
                    // Add a stack if it doesn't yet exist
                    while index >= stacks.len() {
                        stacks.push(Stack::new());
                    }
                    // Ignore empty areas in the stack, and ignore the ` 1   2   3 ...` footer
                    if bytes[0] != b' ' {
                        stacks[index].push(bytes[1]);
                    }
                });
        });
    stacks
}

fn parse_procedures(s: &str) -> Vec<Procedure> {
    s.split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|v| {
            (
                v[1].parse::<usize>().expect("Cannot parse move amount"),
                v[3].parse::<usize>().expect("Cannot parse from stack"),
                v[5].parse::<usize>().expect("Cannot parse to stack"),
            )
        })
        .collect::<Vec<_>>()
}

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> (Vec<Stack>, Vec<Procedure>) {
    let contents = fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', ""); // Strip all carriage returns (found on WSL)
    let (stacks, procedures) = contents
        .split_once("\n\n")
        .expect("Cannot split input into stacks and procedures");
    (parse_stacks(stacks), parse_procedures(procedures))
}

fn game_theory(filepath: &str, f: fn(&Stack, usize, usize) -> u8) -> String {
    let (mut stacks, procedures) = read_and_parse(filepath);
    for (move_amount, from, to) in procedures {
        let from_stack = stacks[from - 1].clone();
        let to_stack = &mut (stacks[to - 1]);
        // First push the items to the new stack
        for index in 0..move_amount {
            to_stack.push(f(&from_stack, move_amount, index));
        }
        // Then pop the items from the old stack
        let from_stack = &mut (stacks[from - 1]);
        from_stack.drain(from_stack.len() - move_amount..);
    }
    stacks
        .iter()
        .map(|stack| *stack.last().expect("No crate on the top of this stack") as char)
        .collect::<String>()
}

fn main() {
    let part_01 = |stack: &Stack, _move_amount, index| stack[stack.len() - index - 1];
    let part_02 = |stack: &Stack, move_amount, index| stack[stack.len() - move_amount + index];
    println!("Top crates: {:?}", game_theory("test_input.txt", part_01));
    println!("Top crates: {:?}", game_theory("input.txt", part_01));
    println!("=========================");
    println!("Top crates: {:?}", game_theory("test_input.txt", part_02));
    println!("Top crates: {:?}", game_theory("input.txt", part_02));
}
