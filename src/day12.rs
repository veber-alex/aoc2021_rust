use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Cave {
    name: &'static str,
    small: bool,
}

impl Cave {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            small: name.chars().all(|c| c.is_lowercase()),
        }
    }
}

fn parse_input(input: &'static str) -> HashMap<Cave, Vec<Cave>> {
    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (Cave::new(a), Cave::new(b))
        })
        .for_each(|(a, b)| {
            map.entry(a).or_default().push(b);
            map.entry(b).or_default().push(a);
        });

    map
}

fn walk12a(data: &HashMap<Cave, Vec<Cave>>, visited: &mut Vec<Cave>) -> usize {
    data[visited.last().unwrap()].iter().fold(0, |acc, &c| {
        acc + if c.small && visited.contains(&c) {
            0
        } else if c.name == "end" {
            1
        } else {
            visited.push(c);
            let n = walk12a(data, visited);
            visited.pop();
            n
        }
    })
}

fn day12a(data: &HashMap<Cave, Vec<Cave>>) -> usize {
    let mut visited = vec![Cave::new("start")];
    walk12a(data, &mut visited)
}

fn walk12b(
    data: &HashMap<Cave, Vec<Cave>>,
    visited: &mut Vec<Cave>,
    set: &mut HashSet<Vec<Cave>>,
    twice: Option<Cave>,
) {
    data[visited.last().unwrap()].iter().for_each(|&c| {
        if c.small {
            if twice == Some(c) {
                if visited.iter().filter(|&&cc| cc == c).count() > 1 {
                    return;
                }
            } else if visited.contains(&c) {
                return;
            }
        }

        if c.name == "end" {
            set.insert(visited.clone());
        } else {
            if c.small && twice.is_none() {
                visited.push(c);
                walk12b(data, visited, set, Some(c));
                visited.pop();
            };
            visited.push(c);
            walk12b(data, visited, set, twice);
            visited.pop();
        }
    })
}

fn day12b(data: &HashMap<Cave, Vec<Cave>>) -> usize {
    let mut set = HashSet::new();
    let mut visited = vec![Cave::new("start")];
    walk12b(data, &mut visited, &mut set, None);
    set.len()
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let data = parse_input(input);

    let day12a = day12a(&data);
    assert_eq!(day12a, 5252);
    println!("day12a: {}", day12a);

    let day12b = day12b(&data);
    assert_eq!(day12b, 147784);
    println!("day12b: {}", day12b);
}
