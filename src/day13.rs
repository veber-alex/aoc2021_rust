use std::collections::HashSet;

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<(u16, u16)>, Vec<(&str, u16)>) {
    let (data, inst) = input.split_once("\n\n").unwrap();
    let data = data
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let inst = inst
        .lines()
        .map(|line| {
            let (_, s) = line.rsplit_once(' ').unwrap();
            let (a, b) = s.split_once('=').unwrap();
            (a, b.parse().unwrap())
        })
        .collect();

    (data, inst)
}

fn day13(data: &mut Vec<(u16, u16)>, instructions: &[(&str, u16)]) -> HashSet<(u16, u16)> {
    fn fold<'a>(points: impl Iterator<Item = &'a mut u16>, line: u16) {
        for p in points {
            if *p > line {
                *p = 2 * line - *p;
            }
        }
    }

    for &(inst, n) in instructions {
        match inst {
            "y" => fold(data.iter_mut().map(|(_, y)| y), n),
            "x" => fold(data.iter_mut().map(|(x, _)| x), n),
            _ => unreachable!(),
        }
    }

    data.iter().copied().collect()
}

fn display(set: &HashSet<(u16, u16)>) {
    for y in 0..6 {
        for x in 0..39 {
            if set.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

fn main() {
    let input = include_str!("../inputs/day13.txt");
    let (mut data, inst) = parse_input(input);

    let day13a = day13(&mut data, &inst[0..1]).len();
    assert_eq!(day13a, 731);
    println!("day13a: {}", day13a);

    let day13b = day13(&mut data, &inst[1..]);
    assert_eq!(day13b.len(), 93);
    println!("day13b:");
    display(&day13b);
}
