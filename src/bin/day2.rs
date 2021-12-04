use std::fs;

use anyhow::{bail, Context, Result};

enum Command {
    Forward,
    Up,
    Down,
}

fn parse_input(input: &str) -> Result<Vec<(Command, usize)>> {
    input
        .lines()
        .map(|l| {
            let (cmd, n) = l
                .trim()
                .split_once(' ')
                .context("failed to split command")?;

            let n = n.parse()?;
            let cmd = match cmd {
                "forward" => Command::Forward,
                "up" => Command::Up,
                "down" => Command::Down,
                _ => bail!("unrecognized command"),
            };

            Ok((cmd, n))
        })
        .collect()
}

fn get_position_times_depth(data: &[(Command, usize)]) -> usize {
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

fn get_position_times_depth_with_aim(data: &[(Command, usize)]) -> usize {
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

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day2.txt")?;
    let commands = parse_input(&input)?;

    let score = get_position_times_depth(&commands);
    assert_eq!(score, 1728414);
    println!("Score: {}", score);

    let score = get_position_times_depth_with_aim(&commands);
    assert_eq!(score, 1765720035);
    println!("Score with aim: {}", score);

    Ok(())
}
