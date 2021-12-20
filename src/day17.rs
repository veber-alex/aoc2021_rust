use std::cmp::Ordering;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn take_step(pos: &mut XY, velocity: &mut XY) {
    pos.x += velocity.x;
    pos.y += velocity.y;

    velocity.x += match velocity.x.cmp(&0) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };

    velocity.y -= 1;
}

fn parse_input(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut iter = input.split('=');
    iter.next();
    let (x, y) = (iter.next().unwrap(), iter.next().unwrap());
    let x = x.trim_end_matches(", y");

    let (a, b) = x.split_once("..").unwrap();
    let start_range = a.parse().unwrap()..=b.parse().unwrap();

    let (a, b) = y.split_once("..").unwrap();
    let end_range = a.parse().unwrap()..=b.parse().unwrap();

    (start_range, end_range)
}

fn day17(target_x: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> (i32, i32) {
    let mut max_y = 0;
    let mut count = 0;

    for x in 0..=*target_x.end() {
        for y in *target_y.start()..=target_y.start().abs() {
            let mut position = XY::new(0, 0);
            let mut v = XY::new(x, y);
            let v_start = XY::new(x, y);

            while position.y >= *target_y.start() {
                take_step(&mut position, &mut v);
                if target_y.contains(&position.y) && target_x.contains(&position.x) {
                    max_y = max_y.max(v_start.y);
                    count += 1;
                    break;
                }
            }
        }
    }

    (((max_y * (max_y + 1)) / 2), count)
}

fn main() {
    let input = include_str!("../inputs/day17.txt");
    let (target_x, target_y) = parse_input(input);

    let (day17a, day17b) = day17(target_x, target_y);
    assert_eq!(day17a, 7381);
    println!("day17a: {}", day17a);
    assert_eq!(day17b, 3019);
    println!("day17b: {}", day17b);
}
