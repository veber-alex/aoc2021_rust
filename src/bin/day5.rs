use std::collections::HashMap;

fn parse_point(s: &str) -> (u16, u16) {
    let (a, b) = s.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn parse_and_run<F>(input: &str, mut f: F) -> usize
where
    F: FnMut(u16, u16, u16, u16) -> bool,
{
    let mut points = HashMap::new();
    let mut add_point = |x, y| {
        points.entry((x, y)).and_modify(|v| *v = true).or_default();
    };

    for line in input.lines() {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (mut x1, mut y1) = parse_point(start);
        let (x2, y2) = parse_point(end);

        if f(x1, y1, x2, y2) {
            continue;
        }

        add_point(x1, y1);
        while (x1, y1) != (x2, y2) {
            x1 = x1 + (x2 > x1) as u16 - (x1 > x2) as u16;
            y1 = y1 + (y2 > y1) as u16 - (y1 > y2) as u16;
            add_point(x1, y1);
        }
    }

    points.values().filter(|v| **v).count()
}

fn day5a(x1: u16, y1: u16, x2: u16, y2: u16) -> bool {
    x1 != x2 && y1 != y2
}

fn day5b(_: u16, _: u16, _: u16, _: u16) -> bool {
    false
}

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let day5a = parse_and_run(input, day5a);
    debug_assert_eq!(day5a, 5608);
    println!("day5a: {}", day5a);

    let day5b = parse_and_run(input, day5b);
    debug_assert_eq!(day5b, 20299);
    println!("day5b: {}", day5b);
}
