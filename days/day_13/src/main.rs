use std::cmp::Ordering;
use std::fs;
use std::iter::zip;

// Note: derive PartialEq for us, as `divider_packets.contains` requires `==`
#[derive(PartialEq)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

fn parse_integer(bytes: &[u8], index: usize) -> (Packet, usize) {
    let start = index;
    let mut end = index;
    while (b'0'..=b'9').contains(&bytes[end]) {
        end += 1;
    }
    let s = std::str::from_utf8(&bytes[start..end]).expect("Cannot parse [u8] as str");
    (
        Packet::Integer(s.parse::<usize>().expect("Cannot parse integer in packet")),
        end,
    )
}

fn parse_list(bytes: &[u8], mut index: usize) -> (Packet, usize) {
    let mut packets = Vec::<Packet>::new();
    index += 1; // Skip b'['
    loop {
        match bytes[index] {
            b']' => {
                index += 1; // Skip b']'
                break;
            }
            b',' => {
                index += 1; // Skip b','
            }
            _ => {
                let (packet, i) = parse_packet(bytes, index);
                index = i;
                packets.push(packet);
            }
        }
    }
    (Packet::List(packets), index)
}

fn parse_packet(bytes: &[u8], index: usize) -> (Packet, usize) {
    match bytes[index] {
        b'[' => parse_list(bytes, index),
        _ => parse_integer(bytes, index),
    }
}

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<[Packet; 2]> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split("\n\n")
        .filter(|line| line != &"") // Remove extraneous empty lines
        .map(|line| line.split_once('\n').expect("Cannot split line into pair"))
        .map(|(a, b)| {
            [
                parse_packet(a.as_bytes(), 0).0,
                parse_packet(b.as_bytes(), 0).0,
            ]
        })
        .collect::<Vec<_>>()
}

fn in_right_order(a: &Packet, b: &Packet) -> Ordering {
    match (a, b) {
        // (Integer, Integer)
        (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
        // (List, List)
        (Packet::List(a), Packet::List(b)) => zip(a.iter(), b.iter())
            .map(|(x, y)| in_right_order(x, y))
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(a.len().cmp(&b.len())),
        // (Integer, List)
        (Packet::Integer(a), b) => in_right_order(&Packet::List(vec![Packet::Integer(*a)]), b),
        // (List, Integer)
        (a, b) => in_right_order(b, a).reverse(),
    }
}

fn sum_of_right_orders(filepath: &str) -> usize {
    read_and_parse(filepath)
        .into_iter()
        .enumerate()
        .filter(|(_, [a, b])| in_right_order(a, b) == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum()
}

fn locate_divider_packets(filepath: &str) -> usize {
    let divider_packets = [2, 6]
        .into_iter()
        .map(|val| Packet::List(vec![Packet::List(vec![Packet::Integer(val)])]))
        .collect::<Vec<_>>();
    let packets = read_and_parse(filepath);
    let mut packets = packets
        .iter()
        .flatten()
        .chain(divider_packets.iter())
        .collect::<Vec<_>>();
    packets.sort_by(|a, b| in_right_order(a, b));
    packets
        .iter()
        .enumerate()
        .filter(|&(_, packet)| divider_packets.contains(packet))
        .map(|(index, _)| index + 1)
        .product()
}

fn main() {
    println!("{:?}", sum_of_right_orders("test_input.txt"));
    println!("{:?}", locate_divider_packets("test_input.txt"));
    println!("=========================");
    println!("{:?}", sum_of_right_orders("input.txt"));
    println!("{:?}", locate_divider_packets("input.txt"));
}
