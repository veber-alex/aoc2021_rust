const NUM_OF_BITS: usize = 12;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}

fn day3a(report: &[&str]) -> usize {
    let mut bit_counts = [0; NUM_OF_BITS];

    for line in report {
        for (bit, c) in bit_counts.iter_mut().zip(line.bytes()) {
            match c {
                b'1' => *bit += 1,
                b'0' => {}
                _ => unreachable!(),
            }
        }
    }

    let mut gamma = 0;
    for (power, &count) in bit_counts.iter().rev().enumerate() {
        if count * 2 > report.len() {
            gamma += 1 << power
        }
    }
    let epsilon = gamma ^ ((1 << NUM_OF_BITS) - 1);

    gamma * epsilon
}

fn bit_count(report: &[&str], bit: usize) -> usize {
    report
        .iter()
        .map(|l| match l.as_bytes()[bit] {
            b'1' => 1,
            b'0' => 0,
            _ => unreachable!(),
        })
        .sum()
}

fn filter_rating<F>(mut report: Vec<&str>, mut f: F) -> &str
where
    F: FnMut(&[&str], usize) -> bool,
{
    let mut i = 0;
    loop {
        match report.as_slice() {
            [s] => return s,
            _ => {
                let common = if f(&report, i) { b'1' } else { b'0' };
                report.retain(|s| s.as_bytes()[i] == common);
            }
        }
        i += 1;
    }
}

fn day3b(report: &[&str]) -> usize {
    let oxygen = filter_rating(report.to_vec(), |report, i| {
        bit_count(report, i) * 2 >= report.len()
    });

    let co2 = filter_rating(report.to_vec(), |report, i| {
        bit_count(report, i) * 2 < report.len()
    });

    usize::from_str_radix(oxygen, 2).unwrap() * usize::from_str_radix(co2, 2).unwrap()
}

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let report = parse_input(input);

    let day3a = day3a(&report);
    debug_assert_eq!(day3a, 1092896);
    println!("day3a: {}", day3a);

    let day3b = day3b(&report);
    debug_assert_eq!(day3b, 4672151);
    println!("day3b: {}", day3b);
}
