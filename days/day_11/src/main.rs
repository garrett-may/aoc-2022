use std::fs;
use std::ops::Add; // for `usize::add`
use std::ops::Mul; // for `usize::mul`

type Operation = Box<dyn Fn(usize) -> usize>;

struct Monkey {
    items: Vec<usize>,
    op: Operation,
    divisor: usize,
    if_true: usize,
    if_false: usize,
    inspection_count: usize,
}

impl Monkey {
    fn inspect(&self, worry_level: usize) -> usize {
        (self.op)(worry_level)
    }

    fn test(&self, worry_level: usize) -> usize {
        if worry_level % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

fn parse_monkey(lines: Vec<&str>) -> Monkey {
    let items = lines[1]
        .split(' ')
        .skip(4)
        .map(|item| item.replace(',', ""))
        .map(|item| item.parse::<usize>().expect("Cannot parse item as usize"))
        .collect::<Vec<_>>();

    let op_info = lines[2].split(' ').skip(6).collect::<Vec<_>>();
    let operator = if op_info[0] == "+" {
        usize::add
    } else {
        usize::mul
    };
    let op: Operation = if let Ok(val) = op_info[1].parse::<usize>() {
        Box::new(move |old| operator(old, val))
    } else {
        Box::new(move |old| operator(old, old))
    };

    let [divisor, if_true, if_false] = [3, 4, 5].map(|index| {
        lines[index]
            .split(' ')
            .last()
            .expect("Cannot get last from line {index}")
            .parse::<usize>()
            .expect("Cannot parse line {index} as usize")
    });

    Monkey {
        items,
        op,
        divisor,
        if_true,
        if_false,
        inspection_count: 0,
    }
}

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<Monkey> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split("\n\n")
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| parse_monkey(line.split('\n').collect::<Vec<_>>()))
        .collect::<Vec<_>>()
}

fn round(monkeys: &mut Vec<Monkey>, lcm: usize, f: fn(usize, usize) -> usize) {
    for i in 0..monkeys.len() {
        let items = monkeys[i].items.clone();
        for item in items {
            let worry_level = f(monkeys[i].inspect(item), lcm);
            monkeys[i].inspection_count += 1;
            let id = monkeys[i].test(worry_level);
            monkeys[id].items.push(worry_level);
        }
        monkeys[i].items.clear();
    }
}

fn monkey_business(filepath: &str, rounds: usize, f: fn(usize, usize) -> usize) -> usize {
    let mut monkeys = read_and_parse(filepath);
    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.divisor)
        .product::<usize>();
    for _ in 0..rounds {
        round(&mut monkeys, lcm, f);
    }
    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();
    inspection_counts.sort_by(|a, b| b.cmp(a));
    inspection_counts[0] * inspection_counts[1]
}

fn main() {
    // Note: We can use the Lowest Common Multiplier (LCM) of the monkeys' divisors
    // in order to make the worry level smaller and therefore prevent overflowing.
    // We don't care about the actual worry level; we only care that we can still
    // test the item.
    //
    // Initially for Part 2 a `Vec<Vec<usize>>` was used. For each monkey and for each
    // item, the remainder of the divisor was stored in order to keep prevent overflowing.
    // That method was objectively slower than using the LCM. However, note that if the
    // LCM is quite a large number, then we might overflow before doing `% lcm` i.e. our
    // method for making the number smaller. Meanwhile, the `Vec<Vec<usize>>` approach only
    // really has to make sure that the maximum divisor of all the monkeys' divisors is not
    // a very large number, which is more reasonable.
    //
    // We will keep the LCM method here as it is better in terms of time and space.
    let part_01 = |worry_level: usize, _: usize| worry_level / 3;
    let part_02 = |worry_level: usize, lcm: usize| worry_level % lcm;

    println!(
        "Monkey business: {}",
        monkey_business("test_input.txt", 20, part_01)
    );

    println!(
        "Monkey business: {}",
        monkey_business("test_input.txt", 10000, part_02)
    );

    println!("=========================");

    println!(
        "Monkey business: {}",
        monkey_business("input.txt", 20, part_01)
    );
    println!(
        "Monkey business: {}",
        monkey_business("input.txt", 10000, part_02)
    );
}
