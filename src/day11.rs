use std::collections::HashSet;
use std::iter;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn parse_input(input: &str) -> Vec<[u8; WIDTH]> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().map(|c| c.to_digit(10).unwrap() as u8);
            [(); WIDTH].map(|_| iter.next().unwrap())
        })
        .collect()
}

fn day11(data: &mut [[u8; WIDTH]; HEIGHT]) -> (usize, usize) {
    let mut count_at_100 = 0;
    let mut step = 1;
    let mut all_flashed = HashSet::new();
    let mut flashed = vec![];

    loop {
        all_flashed.clear();
        flashed.clear();

        (0..HEIGHT)
            .flat_map(|y| iter::repeat(y).zip(0..WIDTH))
            .for_each(|(y, x)| data[y][x] += 1);

        let mut done = false;
        while !done {
            done = true;

            (0..HEIGHT)
                .flat_map(|y| iter::repeat(y).zip(0..WIDTH))
                .for_each(|(y, x)| {
                    if data[y][x] > 9 && !all_flashed.contains(&(y, x)) {
                        flashed.push((y, x));
                        done = false;
                    }
                });

            flashed.iter().for_each(|&(y, x)| {
                let up = y != 0;
                let down = y != HEIGHT - 1;
                let left = x != 0;
                let right = x != WIDTH - 1;

                if up {
                    data[y - 1][x] += 1;
                    if right {
                        data[y - 1][x + 1] += 1;
                    }
                    if left {
                        data[y - 1][x - 1] += 1;
                    }
                }
                if down {
                    data[y + 1][x] += 1;
                    if right {
                        data[y + 1][x + 1] += 1;
                    }
                    if left {
                        data[y + 1][x - 1] += 1;
                    }
                }
                if right {
                    data[y][x + 1] += 1;
                }
                if left {
                    data[y][x - 1] += 1;
                }
            });

            all_flashed.extend(flashed.drain(..));
        }

        all_flashed.iter().for_each(|&(y, x)| data[y][x] = 0);

        if step < 101 {
            count_at_100 += all_flashed.len();
        }
        if all_flashed.len() == WIDTH * HEIGHT {
            return (count_at_100, step);
        }
        step += 1;
    }
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let mut data = parse_input(input);
    let data = data.as_mut_slice().try_into().unwrap();

    let (day11a, day11b) = day11(data);
    assert_eq!(day11a, 1749);
    println!("day11a: {:?}", day11a);
    assert_eq!(day11b, 285);
    println!("day11b: {:?}", day11b);
}
