use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

fn main() {
    part1();
    part2();
}

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part1() {
    let sacks = read_from_file("input.txt");
    let priority_map = build_letter_priority_map();
    let mut total_priority = 0;
    for sack in sacks.lines() {
        total_priority += calculate_sack_priority(sack, &priority_map);
    }
    println!("Total priority: {}", total_priority);
}

fn part2() {
    let sacks = read_from_file("input.txt");
    let priority_map = build_letter_priority_map();

    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for sack in sacks.lines() {
        current_group.push(sack);
        if current_group.len() == 3 {
            groups.push(current_group);
            current_group = Vec::new();
        }
    }

    let mut total_priority = 0;
    for group in groups {
        let common_letter = calculate_common_letter(&group);
        total_priority += priority_map[&common_letter];
    }

    println!("Total badge priority: {}", total_priority);
}

fn build_letter_priority_map() -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for (i, letter) in LETTERS.chars().enumerate() {
        map.insert(letter, i + 1);
    }
    map
}

fn calculate_sack_priority(sack: &str, priority_map: &HashMap<char, usize>) -> usize {
    let len = sack.len();
    let left = &sack[0..len / 2];
    let right = &sack[len / 2..len];

    let left_set: HashSet<char> = HashSet::from_iter(left.chars());

    for letter in right.chars() {
        if left_set.contains(&letter) {
            return priority_map[&letter];
        }
    }

    panic!("No duplicate item found in sack {}", sack);
}

fn calculate_common_letter(sacks: &Vec<&str>) -> char {
    let mut letter_freq = HashMap::new();

    for sack in sacks {
        let unique_chars: HashSet<char> = HashSet::from_iter(sack.chars());
        for letter in unique_chars {
            let freq = letter_freq.entry(letter).or_insert(0);
            *freq += 1;
        }
    }

    for letter in LETTERS.chars() {
        if !letter_freq.contains_key(&letter) {
            continue
        }

        if letter_freq[&letter] == sacks.len() {
            println!("Sacks: {:?}, common letter: {}", sacks, letter);
            return letter;
        }
    }

    panic!("No common letter found in sacks {:?}", sacks);
}

fn read_from_file(input: &str) -> String {
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    contents
}
