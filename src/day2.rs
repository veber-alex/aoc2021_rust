enum Command {
    Forward,
    Up,
    Down,
}

fn parse_input(input: &str) -> Vec<(Command, usize)> {
    input
        .lines()
        .map(|l| {
            let (cmd, n) = l.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            let cmd = match cmd {
                "forward" => Command::Forward,
                "up" => Command::Up,
                "down" => Command::Down,
                _ => unreachable!(),
            };

            (cmd, n)
        })
        .collect()
}

fn day2a(data: &[(Command, usize)]) -> usize {
    let mut position = 0;
    let mut depth = 0;

    for (cmd, n) in data {
        match cmd {
            Command::Forward => position += n,
            Command::Up => depth -= n,
            Command::Down => depth += n,
        }
    }

    position * depth
}

fn day2b(data: &[(Command, usize)]) -> usize {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (cmd, n) in data {
        match cmd {
            Command::Forward => {
                position += n;
                depth += aim * n
            }
            Command::Up => aim -= n,
            Command::Down => aim += n,
        }
    }

    position * depth
}

fn main() {
    let input = include_str!("../inputs/day2.txt");
    let commands = parse_input(input);

    let day2a = day2a(&commands);
    assert_eq!(day2a, 1728414);
    println!("day2a: {}", day2a);

    let day2b = day2b(&commands);
    assert_eq!(day2b, 1765720035);
    println!("day2b: {}", day2b);
}
