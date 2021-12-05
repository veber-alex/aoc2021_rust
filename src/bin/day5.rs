use std::collections::HashMap;

fn parse_point(s: &str) -> (u32, u32) {
    let (a, b) = s.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn parse_and_run<F>(input: &str, mut f: F) -> usize
where
    F: FnMut(u32, u32, u32, u32, &mut HashMap<(u32, u32), bool>),
{
    let mut points = HashMap::new();

    for line in input.lines() {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (xs, ys) = parse_point(start);
        let (xe, ye) = parse_point(end);

        f(xs, ys, xe, ye, &mut points);
    }

    points.values().filter(|v| **v).count()
}

fn day5a(xs: u32, ys: u32, xe: u32, ye: u32, points: &mut HashMap<(u32, u32), bool>) {
    use std::cmp::Ordering::*;

    let mut add_point = |x, y| {
        points.entry((x, y)).and_modify(|v| *v = true).or_default();
    };
    match (xs.cmp(&xe), ys.cmp(&ye)) {
        (Less, Equal) => (xs..=xe).for_each(|x| add_point(x, ys)),
        (Greater, Equal) => (xe..=xs).for_each(|x| add_point(x, ys)),
        (Equal, Less) => (ys..=ye).for_each(|y| add_point(xs, y)),
        (Equal, Greater) => (ye..=ys).for_each(|y| add_point(xs, y)),
        _ => {}
    }
}

fn day5b(xs: u32, ys: u32, xe: u32, ye: u32, points: &mut HashMap<(u32, u32), bool>) {
    use std::cmp::Ordering::*;

    let mut add_point = |x, y| {
        points.entry((x, y)).and_modify(|v| *v = true).or_default();
    };
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

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let day5a = parse_and_run(input, day5a);
    debug_assert_eq!(day5a, 5608);
    println!("day5a: {}", day5a);

    let day5b = parse_and_run(input, day5b);
    debug_assert_eq!(day5b, 20299);
    println!("day5b: {}", day5b);
}
