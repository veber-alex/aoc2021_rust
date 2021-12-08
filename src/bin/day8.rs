fn day8a(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_once('|').unwrap().1.split_ascii_whitespace())
        .fold(0, |acc, s| {
            acc + [2, 3, 4, 7].contains(&s.as_bytes().len()) as usize
        })
}

fn has_bytes_of(source: &[u8], bytes: &[u8]) -> bool {
    bytes.iter().all(|b| source.contains(&b))
}

fn day8b(input: &str) -> usize {
    let mut total = 0;
    for (input, output) in input.lines().map(|line| line.split_once('|').unwrap()) {
        let mut decoded = [&[][..]; 10];

        for s in input.split_ascii_whitespace().map(|s| s.as_bytes()) {
            match s.len() {
                2 => decoded[1] = s,
                4 => decoded[4] = s,
                3 => decoded[7] = s,
                7 => decoded[8] = s,
                _ => continue,
            }
        }

        for s in input
            .split_ascii_whitespace()
            .map(|s| s.as_bytes())
            .filter(|s| s.len() == 6)
        {
            match (has_bytes_of(s, &decoded[4]), has_bytes_of(s, &decoded[1])) {
                (true, _) => decoded[9] = s,
                (false, true) => decoded[0] = s,
                (false, false) => decoded[6] = s,
            }
        }

        for s in input
            .split_ascii_whitespace()
            .map(|s| s.as_bytes())
            .filter(|s| s.len() == 5)
        {
            if has_bytes_of(s, &decoded[1]) {
                decoded[3] = s;
            } else if has_bytes_of(&decoded[9], s) {
                decoded[5] = s;
            } else {
                decoded[2] = s;
            }
        }

        total += output
            .split_ascii_whitespace()
            .map(|s| s.as_bytes())
            .rev()
            .enumerate()
            .fold(0, |acc, (i, out)| {
                acc + decoded
                    .iter()
                    .position(|s| s.len() == out.len() && has_bytes_of(out, s))
                    .unwrap()
                    * 10_usize.pow(i as u32)
            });
    }

    total
}

fn main() {
    let input = include_str!("../../inputs/day8.txt");

    let day8a = day8a(input);
    debug_assert_eq!(day8a, 301);
    println!("day7a: {}", day8a);

    let day8b = day8b(input);
    debug_assert_eq!(day8b, 908067);
    println!("day8b: {}", day8b);
}
