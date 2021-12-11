use std::collections::HashSet;
use std::iter;

const WIDTH: usize = 100;
const LINES: usize = 100;

fn parse_input(input: &str) -> Vec<[u32; WIDTH]> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().map(|c| c.to_digit(10).unwrap());
            [(); WIDTH].map(|_| iter.next().unwrap())
        })
        .collect()
}

fn get_low_points(data: &[[u32; WIDTH]; LINES]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..LINES)
        .flat_map(|y| iter::repeat(y).zip(0..WIDTH))
        .filter_map(|(y, x)| {
            let current = data[y][x];
            ((x == 0 || current < data[y][x - 1])
                && (x == WIDTH - 1 || current < data[y][x + 1])
                && (y == 0 || current < data[y - 1][x])
                && (y == LINES - 1 || current < data[y + 1][x]))
                .then(|| (y, x))
        })
}

fn day9a(data: &[[u32; WIDTH]; LINES]) -> u32 {
    get_low_points(data).fold(0, |acc, (y, x)| acc + 1 + data[y][x])
}

fn walk_point(
    data: &[[u32; WIDTH]; LINES],
    y: usize,
    x: usize,
    found: &mut HashSet<(usize, usize)>,
) {
    let current = data[y][x];

    [
        (x != 0).then(|| (y, x - 1)),
        (x != WIDTH - 1).then(|| (y, x + 1)),
        (y != 0).then(|| (y - 1, x)),
        (y != LINES - 1).then(|| (y + 1, x)),
    ]
    .into_iter()
    .flatten()
    .filter(|&(y, x)| data[y][x] != 9 && data[y][x] > current)
    .for_each(|(y, x)| {
        found.insert((y, x));
        walk_point(data, y, x, found)
    });
}

fn day9b(data: &[[u32; WIDTH]; LINES]) -> usize {
    let mut found = HashSet::new();
    let mut basins: Vec<_> = get_low_points(data)
        .map(|(y, x)| {
            walk_point(data, y, x, &mut found);
            let len = found.len();
            found.clear();
            len + 1
        })
        .collect();
    basins.sort_unstable();

    basins.into_iter().rev().take(3).product()
}

fn main() {
    let input = include_str!("../inputs/day9.txt");
    let data = parse_input(input);
    let data = data.as_slice().try_into().unwrap();

    let day9a = day9a(data);
    assert_eq!(day9a, 545);
    println!("day9a: {}", day9a);

    let day9b = day9b(data);
    assert_eq!(day9b, 950600);
    println!("day9a: {}", day9b);
}
