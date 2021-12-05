use std::collections::HashMap;

fn parse_point(s: &str) -> (u32, u32) {
    let (a, b) = s.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn day5a(input: &str) -> usize {
    use std::cmp::Ordering::*;

    let mut points: HashMap<(u32, u32), usize> = HashMap::new();
    let mut add_point = |x, y| *points.entry((x, y)).or_default() += 1;

    for line in input.lines() {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (xs, ys) = parse_point(start);
        let (xe, ye) = parse_point(end);

        match (xs.cmp(&xe), ys.cmp(&ye)) {
            (Less, Equal) => (xs..=xe).for_each(|x| add_point(x, ys)),
            (Greater, Equal) => (xe..=xs).for_each(|x| add_point(x, ys)),
            (Equal, Less) => (ys..=ye).for_each(|y| add_point(xs, y)),
            (Equal, Greater) => (ye..=ys).for_each(|y| add_point(xs, y)),
            _ => continue,
        };
    }

    points.values().filter(|v| **v > 1).count()
}

fn day5b(input: &str) -> usize {
    use std::cmp::Ordering::*;

    let mut points: HashMap<(u32, u32), usize> = HashMap::new();
    let mut add_point = |x, y| *points.entry((x, y)).or_default() += 1;

    for line in input.lines() {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (xs, ys) = parse_point(start);
        let (xe, ye) = parse_point(end);

        match (xs.cmp(&xe), ys.cmp(&ye)) {
            (Less, Equal) => (xs..=xe).for_each(|x| add_point(x, ys)),
            (Greater, Equal) => (xe..=xs).for_each(|x| add_point(x, ys)),
            (Equal, Less) => (ys..=ye).for_each(|y| add_point(xs, y)),
            (Equal, Greater) => (ye..=ys).for_each(|y| add_point(xs, y)),
            (Less, Less) => (xs..=xe).zip(ys..=ye).for_each(|(x, y)| add_point(x, y)),
            (Less, Greater) => (xs..=xe)
                .zip((ye..=ys).rev())
                .for_each(|(x, y)| add_point(x, y)),
            (Greater, Less) => (xe..=xs)
                .rev()
                .zip(ys..=ye)
                .for_each(|(x, y)| add_point(x, y)),
            (Greater, Greater) => (xe..=xs).zip(ye..=ys).for_each(|(x, y)| add_point(x, y)),
            (Equal, Equal) => unreachable!(),
        };
    }

    points.values().filter(|v| **v > 1).count()
}

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let day5a = day5a(input);
    debug_assert_eq!(day5a, 5608);
    println!("day5a: {}", day5a);

    let day5b = day5b(input);
    debug_assert_eq!(day5b, 20299);
    println!("day5b: {}", day5b);
}
