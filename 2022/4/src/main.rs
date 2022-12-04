use std::fs::File;

fn main() {
    part1();
    part2();
}

type Interval<T> = (T, T);

// In how many assignment pairs does one range fully contain the other?
fn part1() {
    let raw_pairs = read_from_file("input.txt");
    let mut total = 0;

    for raw_pair in raw_pairs.lines() {
        let mut split = raw_pair.split(",");
        let l = split.next().unwrap();
        let r = split.next().unwrap();

        let l_interval = parse_interval(l);
        let r_interval = parse_interval(r);

        if contains(l_interval, r_interval) || contains(r_interval, l_interval) {
            total += 1;
        }
    }

    println!("Total where one pair contains the other: {}", total);
}

// In how many assignment pairs do the ranges overlap?
fn part2() {
    let raw_pairs = read_from_file("input.txt");
    let mut total = 0;

    for raw_pair in raw_pairs.lines() {
        let mut split = raw_pair.split(",");
        let l = split.next().unwrap();
        let r = split.next().unwrap();

        let l_interval = parse_interval(l);
        let r_interval = parse_interval(r);

        // (point, range_open_close)
        // range_open_close=-1 => range opens
        // range_open_close=+1 => range closes
        // tuple sort orders the points as expected
        let mut points = Vec::new();
        points.push((l_interval.0, -1));
        points.push((l_interval.1, 1));
        points.push((r_interval.0, -1));
        points.push((r_interval.1, 1));
        points.sort();

        let mut active_range_count = 0;
        let mut max_active_range_count = 0;

        for (_, range_open_close) in points {
            active_range_count += -1*range_open_close;
            max_active_range_count = std::cmp::max(max_active_range_count, active_range_count);
        }

        if max_active_range_count > 1 {
            total += 1;
        }

    }

    println!("Total where the pairs overlap: {}", total);
}

fn parse_interval(raw: &str) -> Interval<u32> {
    let mut min_max = raw.split("-");
    let min = min_max.next().unwrap().parse::<u32>().unwrap();
    let max = min_max.next().unwrap().parse::<u32>().unwrap();
    (min, max)
}

fn contains<T: PartialOrd>(a: Interval<T>, b: Interval<T>) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}


fn read_from_file(input: &str) -> String {
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    contents
}
