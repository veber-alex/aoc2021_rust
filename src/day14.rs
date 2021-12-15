use std::collections::HashMap;
use std::mem;

fn parse_input(input: &str) -> (Vec<u8>, HashMap<[u8; 2], u8>) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.bytes().collect();
    let rules = rules
        .lines()
        .map(|line| {
            let (pair, insert) = line.split_once(" -> ").unwrap();
            let pair = [pair.as_bytes()[0], pair.as_bytes()[1]];
            let insert = insert.as_bytes()[0];
            (pair, insert)
        })
        .collect();

    (template, rules)
}

fn day14(template: &[u8], rules: &HashMap<[u8; 2], u8>, n: u32) -> u64 {
    let mut cache1: HashMap<[u8; 2], u64> = HashMap::new();
    let mut cache2: HashMap<[u8; 2], u64> = HashMap::new();
    let mut count: HashMap<u8, u64> = HashMap::new();

    for pair in template.windows(2).map(|p| p.try_into().unwrap()) {
        *cache1.entry(pair).or_default() += 1;
        for p in pair {
            *count.entry(p).or_default() += 1;
        }
    }

    for _ in 0..n {
        for (k, &v) in cache1.iter() {
            let mid = rules[k];
            *cache2.entry([k[0], mid]).or_default() += v;
            *cache2.entry([mid, k[1]]).or_default() += v;
            *count.entry(mid).or_default() += v;
        }
        cache1.clear();
        mem::swap(&mut cache1, &mut cache2);
    }

    count.values().max().unwrap() - count.values().min().unwrap()
}

fn main() {
    let input = include_str!("../inputs/day14.txt");
    let (template, rules) = parse_input(input);

    let day14a = day14(&template, &rules, 10);
    assert_eq!(day14a, 2891);
    println!("day14a: {}", day14a);

    let day14b = day14(&template, &rules, 40);
    assert_eq!(day14b, 4607749009683);
    println!("day14b: {}", day14b);
}
