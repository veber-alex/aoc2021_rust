fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn day7a(data: &[u32]) -> u32 {
    data.iter()
        .copied()
        .map(|pos| data.iter().copied().map(|v| pos.max(v) - pos.min(v)).sum())
        .min()
        .unwrap()
}

fn day7b(data: &[u32]) -> u32 {
    (1..)
        .scan(u32::MAX, |best, pos| {
            let d = data
                .iter()
                .copied()
                .map(|v| {
                    let n = pos.max(v) - pos.min(v);
                    n * (n + 1) / 2
                })
                .sum();
            (d < *best).then(|| {
                *best = d;
                d
            })
        })
        .last()
        .unwrap()
}

fn main() {
    let input = include_str!("../inputs/day7.txt");
    let data = parse_input(input);

    let day7a = day7a(&data);
    assert_eq!(day7a, 343441);
    println!("day7a: {}", day7a);

    let day7b = day7b(&data);
    assert_eq!(day7b, 98925151);
    println!("day7b: {}", day7b);
}
