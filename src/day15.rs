use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn parse_input(input: &str) -> (Vec<u8>, usize) {
    let mut line_count = 0;
    let data = input
        .lines()
        .inspect(|_| line_count += 1)
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect();

    (data, line_count)
}

struct Vertex {
    id: usize,
    priority: Reverse<u32>,
}

impl Vertex {
    fn new(id: usize, priority: u32) -> Self {
        Self {
            id,
            priority: Reverse(priority),
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

fn day15(data: &[u8], lines: usize) -> usize {
    let mut q = BinaryHeap::with_capacity(data.len());
    let mut dist = vec![usize::MAX; data.len()];
    let target = data.len() - 1;

    dist[0] = 0;

    q.push(Vertex::new(0, 0));

    loop {
        let Vertex { id: u, .. } = q.pop().unwrap();

        if u == target {
            break *dist.last().unwrap();
        }

        [
            (u + lines < data.len()).then(|| u + lines),
            (u + 1 % lines != 0).then(|| u + 1),
            u.checked_sub(lines),
            (u % lines != 0).then(|| u - 1),
        ]
        .into_iter()
        .flatten()
        .for_each(|v| {
            let cost = data[v] as usize;
            let alt = dist[u] + cost;
            if alt < dist[v] {
                dist[v] = alt;
                q.push(Vertex::new(v, alt as u32));
            }
        })
    }
}

fn multiply_data(data: &[u8], lines: usize, multi: usize) -> (Vec<u8>, usize) {
    let new_lines = lines * multi;
    let new_data = (0..new_lines.pow(2))
        .map(|v| {
            let d = (v % lines) + (v % (new_lines * lines) / new_lines) * lines;
            let cost =
                (data[d] + (v % new_lines / lines) as u8 + (v / new_lines / lines) as u8) % 9;
            (cost == 0).then(|| 9).unwrap_or(cost)
        })
        .collect::<Vec<_>>();

    (new_data, new_lines)
}

fn main() {
    let input = include_str!("../inputs/day15.txt");
    let (data, lines) = parse_input(input);

    let day15a = day15(&data, lines);
    assert_eq!(day15a, 386);
    println!("day15a: {}", day15a);

    let (data, lines) = multiply_data(&data, lines, 5);
    let day15b = day15(&data, lines);
    assert_eq!(day15b, 2806);
    println!("day15b: {}", day15b);
}
