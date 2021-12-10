fn day10a(input: &str) -> usize {
    let mut stack = vec![];
    input.lines().fold(0, |acc, line| {
        let mut err = 0;
        for b in line.bytes() {
            match b {
                b'(' | b'{' | b'<' | b'[' => stack.push(b),
                b']' if stack.pop() != Some(b'[') => {
                    err = 57;
                    break;
                }
                b'}' if stack.pop() != Some(b'{') => {
                    err = 1197;
                    break;
                }
                b'>' if stack.pop() != Some(b'<') => {
                    err = 25137;
                    break;
                }
                b')' if stack.pop() != Some(b'(') => {
                    err = 3;
                    break;
                }
                _ => {}
            }
        }
        stack.clear();

        acc + err
    })
}

fn day10b(input: &str) -> usize {
    let mut stack = vec![];
    let mut scores = input
        .lines()
        .filter_map(|line| {
            stack.clear();
            for b in line.bytes() {
                match b {
                    b'(' | b'{' | b'<' | b'[' => stack.push(b),
                    b']' if stack.pop() != Some(b'[') => return None,
                    b'}' if stack.pop() != Some(b'{') => return None,
                    b'>' if stack.pop() != Some(b'<') => return None,
                    b')' if stack.pop() != Some(b'(') => return None,
                    _ => {}
                }
            }

            stack
                .iter()
                .rev()
                .fold(0, |acc, b| {
                    acc * 5
                        + match b {
                            b'(' => 1,
                            b'[' => 2,
                            b'{' => 3,
                            b'<' => 4,
                            _ => unreachable!(),
                        }
                })
                .into()
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("../inputs/day10.txt");

    let day10a = day10a(input);
    assert_eq!(day10a, 462693);
    println!("day10a: {}", day10a);

    let day10b = day10b(input);
    assert_eq!(day10b, 3094671161);
    println!("day10b: {}", day10b);
}
