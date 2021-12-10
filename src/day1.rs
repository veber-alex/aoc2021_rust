fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn day1a(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[1] > w[0]).count()
}

fn day1b(depths: &[usize]) -> usize {
    depths.windows(4).filter(|w| w[3] > w[0]).count()
}

fn main() {
    let input = include_str!("../inputs/day1.txt");
    let depths = parse_input(input);

    let day1a = day1a(&depths);
    assert_eq!(day1a, 1759);
    println!("day1a: {}", day1a);

    let day1b = day1b(&depths);
    assert_eq!(day1b, 1805);
    println!("day1b: {}", day1b);
}
