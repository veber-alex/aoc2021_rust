use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

type Point = (i16, i16, i16);

const ROTATIONS: [fn(Point) -> Point; 24] = [
    |(x, y, z)| (x, y, z),
    |(x, y, z)| (x, -z, y),
    |(x, y, z)| (x, -y, -z),
    |(x, y, z)| (x, z, -y),
    |(x, y, z)| (y, -z, -x),
    |(x, y, z)| (y, -x, z),
    |(x, y, z)| (y, x, -z),
    |(x, y, z)| (y, z, x),
    |(x, y, z)| (z, -y, x),
    |(x, y, z)| (z, -x, -y),
    |(x, y, z)| (z, x, y),
    |(x, y, z)| (z, y, -x),
    |(x, y, z)| (-x, -z, -y),
    |(x, y, z)| (-x, -y, z),
    |(x, y, z)| (-x, y, -z),
    |(x, y, z)| (-x, z, y),
    |(x, y, z)| (-y, -z, x),
    |(x, y, z)| (-y, z, -x),
    |(x, y, z)| (-y, -x, -z),
    |(x, y, z)| (-y, x, z),
    |(x, y, z)| (-z, -y, -x),
    |(x, y, z)| (-z, y, x),
    |(x, y, z)| (-z, -x, y),
    |(x, y, z)| (-z, x, -y),
];

#[derive(Debug, Clone)]
struct Scanner {
    id: u8,
    location: Point,
    data: Vec<Point>,
}

impl Scanner {
    fn new(data: Vec<Point>, id: u8) -> Self {
        Self {
            id,
            data,
            location: (0, 0, 0),
        }
    }
}

fn normalize((x, y, z): Point, (xn, yn, zn): Point) -> Point {
    (x + xn, y + yn, z + zn)
}

fn find_scanner_overlap(
    s1: &Scanner,
    s2: &mut Scanner,
    unique_beacons: &mut HashSet<Point>,
    counter: &mut HashMap<Point, u8>,
) -> bool {
    for &rotf in ROTATIONS.iter() {
        counter.clear();
        let s2_rotated = s2.data.iter().copied().map(rotf).collect::<Vec<_>>();
        for &(p1x, p1y, p1z) in &s1.data {
            for (p2x, p2y, p2z) in &s2_rotated {
                let possible_s2_location = (p1x - p2x, p1y - p2y, p1z - p2z);
                let count = counter.entry(possible_s2_location).or_default();
                *count += 1;
                if *count == 12 {
                    s2.data = s2_rotated;
                    s2.location = normalize(possible_s2_location, s1.location);
                    unique_beacons.extend(s2.data.iter().map(|&p| normalize(p, s2.location)));
                    return true;
                }
            }
        }
    }

    false
}

fn parse_input(input: &str) -> VecDeque<Scanner> {
    let mut id = 0;
    input
        .split("\n\n")
        .map(|lines| {
            let points = lines
                .lines()
                .skip(1)
                .map(|line| {
                    let mut iter = line.split(',');
                    (
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                    )
                })
                .collect();
            id += 1;
            Scanner::new(points, id)
        })
        .collect()
}

fn day19a(scanners: &mut VecDeque<Scanner>) -> usize {
    let mut unique_beacons: HashSet<Point> = HashSet::new();
    let mut counter = HashMap::new();
    let mut tested_pairs = HashSet::new();

    let s0 = scanners.pop_front().unwrap();
    unique_beacons.extend(&s0.data);

    let mut found_scanners = vec![s0];

    while let Some(mut scanner) = scanners.pop_front() {
        #[allow(clippy::never_loop)]
        'out: loop {
            for found_scanner in &found_scanners {
                if !tested_pairs.contains(&(scanner.id, found_scanner.id))
                    && find_scanner_overlap(
                        found_scanner,
                        &mut scanner,
                        &mut unique_beacons,
                        &mut counter,
                    )
                {
                    found_scanners.push(scanner);
                    break 'out;
                } else {
                    tested_pairs.insert((scanner.id, found_scanner.id));
                }
            }
            scanners.push_back(scanner);
            break;
        }
    }
    scanners.extend(found_scanners);

    unique_beacons.len()
}

fn day19b(scanners: &VecDeque<Scanner>) -> i16 {
    scanners
        .iter()
        .flat_map(|s| iter::repeat(s).zip(scanners.iter()))
        .map(|(s1, s2)| {
            let (s1x, s1y, s1z) = s1.location;
            let (s2x, s2y, s2z) = s2.location;
            (s1x - s2x).abs() + (s1y - s2y).abs() + (s1z - s2z).abs()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../inputs/day19.txt");
    let mut scanners = parse_input(input);

    let day19a = day19a(&mut scanners);
    assert_eq!(day19a, 465);
    println!("day19a: {}", day19a);

    let day19b = day19b(&scanners);
    assert_eq!(day19b, 12149);
    println!("day19b: {}", day19b);
}
