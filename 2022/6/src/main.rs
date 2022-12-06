use std::fs::File;
use std::collections::{VecDeque, HashSet};
use std::iter::FromIterator;

fn main() {
    part1();
    part2();
}

const START_OF_PACKET_MARKER_LENGTH: usize = 4;
const START_OF_MESSAGE_MARKER_LENGTH: usize = 14;


fn find_protocol_header(signal: &str, marker_length: usize) -> usize {
    let mut last_4_digits = VecDeque::new();
    for (i, c) in signal.chars().enumerate() {
        last_4_digits.push_back(c);
        if last_4_digits.len() < marker_length {
            continue;
        }

        if last_4_digits.len() > marker_length {
            last_4_digits.pop_front();
        }

        if last_4_digits.len() == marker_length {
            let set: HashSet<&char> = HashSet::from_iter(last_4_digits.iter());
            if set.len() == marker_length {
                println!("last {} chars were the same at index {}", marker_length, i);
                println!("{}: {}", i, last_4_digits.iter().collect::<String>());
                return i + 1
            }
        }
    }

    return 0
}

fn part1() {
    println!("{}", find_protocol_header(&read_from_file("input.txt"), START_OF_PACKET_MARKER_LENGTH));
}

fn part2() {
    println!("{}", find_protocol_header(&read_from_file("input.txt"), START_OF_MESSAGE_MARKER_LENGTH));
}

fn read_from_file(input: &str) -> String {
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    contents
}

