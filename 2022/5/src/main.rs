use std::fs::File;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let raw_input = read_from_file("input.txt");
    let mut stacks = parse_stacks(&raw_input);
    parse_and_make_moves(&mut stacks, &raw_input);
    let top_of_stacks = stacks.iter().map(|stack| stack.peek().unwrap()).collect::<Vec<_>>();
    println!("top of stacks for cratemover 9000");
    println!("{:?}", top_of_stacks);
}


fn part2() {
    let raw_input = read_from_file("input.txt");
    let mut stacks = parse_stacks(&raw_input);
    parse_and_make_moves_2(&mut stacks, &raw_input);
    let top_of_stacks = stacks.iter().map(|stack| stack.peek().unwrap()).collect::<Vec<_>>();
    println!("top of stacks for cratemover 9001");
    println!("{:?}", top_of_stacks);
}

#[derive(Debug)]
struct Stack <T> {
    stack: Vec<T>,
}

impl <T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

// implement display for stack
impl <T: std::fmt::Display> std::fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for item in self.stack.iter() {
            s.push_str(&format!("{} ", item));
        }
        write!(f, "{}", s)
    }
}


type Stacks<T> = Vec<Stack<T>>;

fn parse_stacks(input: &str) -> Stacks<String> {
    let mut stacks: Stacks<String> = Vec::new();
    let mut raw_stacks = Vec::new();

    let mut stack_count = 0;

    // build the stacks
    for line in input.lines() {
        if !line.starts_with(" 1") {
            raw_stacks.push(line);
            continue;
        }

        let stack_list = line.trim().split("   ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        for stack_num in stack_list {
            stacks.push(Stack::new());
            stack_count = std::cmp::max(stack_count, stack_num);
        }
        break;
    }

    // the last stack in the list is the bottom, so lets build the stack
    // in reverse order.
    for raw_stack in raw_stacks.iter().rev() {
        let mut i = 0;
        let mut curr_stack = 0;

        while i < raw_stack.len() {
            // take first 3 chars
            let stack_item = raw_stack[i..i+3].trim().to_string();

            if stack_item.starts_with("[") {
                stacks[curr_stack].push(stack_item);
            }

            i += 4;
            curr_stack += 1;
        }
    }

    stacks
}

fn parse_and_make_moves(stacks: &mut Stacks<String>, input: &str) {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in input.lines() {
        let maybe_caps = move_re.captures(line);

        if maybe_caps.is_none() {
            continue;
        }

        let caps = maybe_caps.unwrap();
        let move_count = caps[1].parse::<usize>().unwrap();
        let move_from = caps[2].parse::<usize>().unwrap();
        let move_to = caps[3].parse::<usize>().unwrap();
        make_moves(stacks, move_from, move_to, move_count);
    }
}

fn make_moves(stacks: &mut Stacks<String>, from: usize, to: usize, count: usize) {
    let mut remaining = count;

    let from_i = from - 1;
    let to_i = to - 1;

    while remaining > 0 {
        let to_move = stacks[from_i].pop().unwrap();
        stacks[to_i].push(to_move);
        remaining -= 1;
    }
}

fn parse_and_make_moves_2(stacks: &mut Stacks<String>, input: &str) {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in input.lines() {
        let maybe_caps = move_re.captures(line);

        if maybe_caps.is_none() {
            continue;
        }

        let caps = maybe_caps.unwrap();
        let move_count = caps[1].parse::<usize>().unwrap();
        let move_from = caps[2].parse::<usize>().unwrap();
        let move_to = caps[3].parse::<usize>().unwrap();
        make_moves_2(stacks, move_from, move_to, move_count);
    }
}

fn make_moves_2(stacks: &mut Stacks<String>, from: usize, to: usize, count: usize) {
    let mut remaining = count;

    let from_i = from - 1;
    let to_i = to - 1;

    let mut removed_group = Vec::new();

    // remove them all from the source stack...
    while remaining > 0 {
        let to_move = stacks[from_i].pop().unwrap();
        removed_group.push(to_move);
        remaining -= 1;
    }

    // place the entire group on top of the target stack
    for item in removed_group.iter().rev() {
        stacks[to_i].push(item.to_string());
    }
}


fn read_from_file(input: &str) -> String {
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    contents
}

