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

fn day1a(data: &[(Command, usize)]) -> usize {
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

fn day1b(data: &[(Command, usize)]) -> usize {
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
    let input = include_str!("../../inputs/day2.txt");
    let commands = parse_input(input);

    let day1a = day1a(&commands);
    debug_assert_eq!(day1a, 1728414);
    println!("day1a: {}", day1a);

    let day1b = day1b(&commands);
    debug_assert_eq!(day1b, 1765720035);
    println!("day1b: {}", day1b);
}
