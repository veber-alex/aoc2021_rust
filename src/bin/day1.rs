use std::fs;

use anyhow::Result;

fn parse_input(input: &str) -> Result<Vec<usize>> {
    input.lines().map(|l| Ok(l.trim().parse()?)).collect()
}

fn num_of_increments(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[1] > w[0]).count()
}

fn num_of_increments_3(depths: &[usize]) -> usize {
    let mut a = usize::MAX;
    depths
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .filter(|&b| {
            let inc = b > a;
            a = b;
            inc
        })
        .count()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day1.txt")?;
    let depths = parse_input(&input)?;

    let increments = num_of_increments(&depths);
    assert_eq!(increments, 1759);
    println!("Single increments: {}", increments);

    let increments_3 = num_of_increments_3(&depths);
    assert_eq!(increments_3, 1805);
    println!("3 increments: {}", increments_3);

    Ok(())
}
