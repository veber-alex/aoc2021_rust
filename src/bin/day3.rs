use std::fs;

use anyhow::{bail, Context, Result};

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.trim()).collect::<Vec<_>>()
}

fn get_power_consumption(report: &[&str]) -> Result<usize> {
    let num_of_bits = report.first().context("empty report")?.bytes().count();
    let mut bit_counts = vec![0; num_of_bits];

    for line in report {
        for (c, bit) in line.bytes().zip(bit_counts.iter_mut()) {
            match c {
                b'1' => *bit += 1,
                b'0' => {}
                _ => bail!("unknown bit symbol"),
            }
        }
    }

    let mut gamma = 0;
    for (power, &count) in bit_counts.iter().rev().enumerate() {
        if count * 2 > report.len() {
            gamma += 2usize.pow(power as u32)
        }
    }
    let epsilon = gamma ^ (2usize.pow(num_of_bits as u32) - 1);

    Ok(gamma * epsilon)
}

fn bit_count(report: &[&str], bit: usize) -> Result<usize> {
    report
        .iter()
        .map(|l| match l.as_bytes()[bit] {
            b'1' => Ok(1),
            b'0' => Ok(0),
            _ => bail!("unknown bit symbol"),
        })
        .sum()
}

fn filter_rating<'a, F>(mut report: Vec<&'a str>, mut f: F) -> Result<&'a str>
where
    F: FnMut(&Vec<&str>, usize) -> Result<bool>,
{
    let mut i = 0;
    loop {
        match report.as_slice() {
            [s] => return Ok(s),
            _ => {
                let common = if f(&report, i)? { b'1' } else { b'0' };
                report.retain(|s| s.as_bytes()[i] == common);
            }
        }
        i += 1;
    }
}

fn get_life_support_rating(report: &[&str]) -> Result<usize> {
    let oxygen = filter_rating(report.to_vec(), |report, i| {
        Ok(bit_count(report, i)? * 2 >= report.len())
    })?;

    let co2 = filter_rating(report.to_vec(), |report, i| {
        Ok(bit_count(report, i)? * 2 < report.len())
    })?;

    Ok(usize::from_str_radix(oxygen, 2)? * usize::from_str_radix(co2, 2)?)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day3.txt")?;
    let report = parse_input(&input);

    let power_consumption = get_power_consumption(&report)?;
    assert_eq!(power_consumption, 1092896);
    println!("Power consumption: {}", power_consumption);

    let life_support_rating = get_life_support_rating(&report)?;
    assert_eq!(life_support_rating, 4672151);
    println!("Life support rating: {}", life_support_rating);

    Ok(())
}
