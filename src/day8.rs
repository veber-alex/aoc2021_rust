fn day8a(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_once('|').unwrap().1.split_ascii_whitespace())
        .fold(0, |acc, s| {
            acc + [2, 3, 4, 7].contains(&s.as_bytes().len()) as usize
        })
}

fn has_bytes_of(target: &str, bytes: &str) -> bool {
    bytes.bytes().all(|b| target.as_bytes().contains(&b))
}

fn day8b(input: &str) -> usize {
    let mut decoded = [""; 10];
    let mut buffer = Vec::with_capacity(10);

    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .fold(0, |acc, (input, output)| {
            buffer.extend(input.split_ascii_whitespace());

            for s in &buffer {
                match s.len() {
                    2 => decoded[1] = s,
                    4 => decoded[4] = s,
                    3 => decoded[7] = s,
                    7 => decoded[8] = s,
                    _ => continue,
                }
            }

            for s in buffer.iter().filter(|s| s.len() == 6) {
                if has_bytes_of(s, decoded[4]) {
                    decoded[9] = s
                } else if has_bytes_of(s, decoded[1]) {
                    decoded[0] = s
                } else {
                    decoded[6] = s
                }
            }

            for s in buffer.iter().filter(|s| s.len() == 5) {
                if has_bytes_of(s, decoded[1]) {
                    decoded[3] = s;
                } else if has_bytes_of(decoded[9], s) {
                    decoded[5] = s;
                } else {
                    decoded[2] = s;
                }
            }

            buffer.clear();

            acc + output
                .split_ascii_whitespace()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, out)| {
                    acc + decoded
                        .iter()
                        .position(|s| s.len() == out.len() && has_bytes_of(out, s))
                        .unwrap()
                        * 10_usize.pow(i as u32)
                })
        })
}

fn main() {
    let input = include_str!("../inputs/day8.txt");

    let day8a = day8a(input);
    assert_eq!(day8a, 301);
    println!("day8a: {}", day8a);

    let day8b = day8b(input);
    assert_eq!(day8b, 908067);
    println!("day8b: {}", day8b);
}
