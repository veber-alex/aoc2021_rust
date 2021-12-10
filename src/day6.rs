use std::ops::Range;

fn parse_input(input: &str) -> [usize; 9] {
    let mut arr = [0; 9];

    for s in input.split(',') {
        arr[s.parse::<usize>().unwrap()] += 1;
    }

    arr
}

fn day6(arr: &mut [usize; 9], gen_range: Range<usize>) -> usize {
    for g in gen_range {
        arr[(g + 7) % 9] += arr[g % 9];
    }

    arr.iter().sum()
}

fn main() {
    let input = include_str!("../inputs/day6.txt");
    let mut arr = parse_input(input);

    let day6a = day6(&mut arr, 0..80);
    assert_eq!(day6a, 359344);
    println!("day6a: {}", day6a);

    let day6b = day6(&mut arr, 80..256);
    assert_eq!(day6b, 1629570219571);
    println!("day6b: {}", day6b);
}
