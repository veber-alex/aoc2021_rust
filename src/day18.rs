use Item::*;

#[derive(Debug, Clone, Copy)]
enum Item {
    Number(u16),
    Start,
    End,
}

impl Item {
    fn number(&mut self) -> &mut u16 {
        match self {
            Number(n) => n,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Pair(Vec<Item>);

impl Pair {
    fn explode(&mut self) -> bool {
        let mut track_before = None;
        let mut track_after = None;
        let mut nested = 0;
        let mut exploded = None;
        let mut iter = self.0.iter().enumerate();

        while let Some((idx, item)) = iter.next() {
            match *item {
                Number(_) => {
                    if exploded.is_none() {
                        track_before = Some(idx);
                    } else {
                        track_after = Some(idx);
                        break;
                    }
                }
                Start => {
                    nested += 1;
                    if nested == 5 && exploded.is_none() {
                        exploded = Some(idx);
                        iter.next();
                        iter.next();
                    }
                }
                End => nested -= 1,
            }
        }

        if let Some(e) = exploded {
            if let Some(before) = track_before {
                let left = *self.0[e + 1].number();
                *self.0[before].number() += left;
            }
            if let Some(after) = track_after {
                let right = *self.0[e + 2].number();
                *self.0[after].number() += right;
            }
            self.0.splice(e..e + 4, Some(Number(0)));
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        for (idx, item) in self.0.iter().enumerate() {
            if let Number(n @ 10..) = *item {
                self.0.splice(
                    idx..idx + 1,
                    [Start, Number(n / 2), Number((n + 1) / 2), End],
                );
                return true;
            }
        }
        false
    }

    fn add(&mut self, other: &Pair) {
        self.0.insert(0, Start);
        self.0.extend_from_slice(&other.0);
        self.0.push(End);
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn magnitude(&self) -> u16 {
        let mut stack = vec![];
        for &item in &self.0 {
            match item {
                End => {
                    let left = stack.pop().unwrap() * 2;
                    *stack.last_mut().unwrap() = *stack.last_mut().unwrap() * 3 + left;
                }
                Number(n) => stack.push(n),
                Start => {}
            }
        }

        stack[0]
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            let v = line
                .chars()
                .filter_map(|c| match c {
                    '[' => Some(Start),
                    ']' => Some(End),
                    ',' => None,
                    n => Some(Number(n.to_digit(10).unwrap() as _)),
                })
                .collect();

            Pair(v)
        })
        .collect()
}

fn day18a(pairs: &mut [Pair]) -> u16 {
    pairs
        .iter_mut()
        .reduce(|p1, p2| {
            p1.add(p2);
            p1.reduce();
            p1
        })
        .unwrap()
        .magnitude()
}

fn day18b(all_pairs: &mut [Pair]) -> u16 {
    let mut max = 0;
    for _ in 0..all_pairs.len() {
        for p in all_pairs.iter().skip(1) {
            let mut pair = all_pairs[0].clone();
            pair.add(p);
            pair.reduce();
            max = max.max(pair.magnitude());
        }
        all_pairs.rotate_left(1);
    }

    max
}

fn main() {
    let input = include_str!("../inputs/day18.txt");

    let mut pairs = parse_input(input);
    let day18a = day18a(&mut pairs.clone());
    assert_eq!(day18a, 3647);
    println!("day18a: {}", day18a);

    let day18b = day18b(&mut pairs);
    assert_eq!(day18b, 4600);
    println!("day18b: {}", day18b);
}
