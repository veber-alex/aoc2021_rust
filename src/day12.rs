use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(a, b)| {
            map.entry(a).or_default().push(b);
            map.entry(b).or_default().push(a);
        });

    map
}

fn walk12a<'a>(data: &HashMap<&'a str, Vec<&'a str>>, visited: &mut Vec<&'a str>) -> u32 {
    data[visited.last().unwrap()].iter().fold(0, |acc, &c| {
        let small = c.bytes().any(|b| b.is_ascii_lowercase());
        acc + if small && visited.contains(&c) {
            0
        } else if c == "end" {
            1
        } else {
            visited.push(c);
            let n = walk12a(data, visited);
            visited.pop();
            n
        }
    })
}

fn day12a(data: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut visited = vec!["start"];
    walk12a(data, &mut visited)
}

fn walk12b<'a>(
    data: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut Vec<&'a str>,
    mut twice: Option<(&str, u8)>,
) -> u32 {
    data[visited.last().unwrap()].iter().fold(0, |acc, &c| {
        if c == "end" {
            return acc
                + match twice {
                    Some((_, 2)) | None => 1,
                    _ => 0,
                };
        };

        let small = c.bytes().any(|b| b.is_ascii_lowercase());

        if small {
            match twice {
                Some((cc, 2)) if cc == c => return acc,
                Some((cc, count)) if cc == c => twice = Some((cc, count + 1)),
                _ => {
                    if visited.contains(&c) {
                        return acc;
                    }
                }
            }
        }

        let mut tw = 0;
        visited.push(c);
        if small && twice.is_none() {
            tw = walk12b(data, visited, Some((c, 1)));
        };
        let n = walk12b(data, visited, twice);
        visited.pop();
        if let Some((cc, count)) = twice {
            if c == cc {
                twice = Some((cc, count - 1))
            }
        }

        acc + n + tw
    })
}

fn day12b(data: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut visited = vec!["start"];
    walk12b(data, &mut visited, None)
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let data = parse_input(input);

    let day12a = day12a(&data);
    assert_eq!(day12a, 5252);
    println!("day12a: {}", day12a);

    let day12b = day12b(&data);
    assert_eq!(day12b, 147784);
    println!("day12b: {}", day12b);
}
