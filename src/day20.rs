use std::collections::VecDeque;

fn parse_input(input: &str) -> (&[u8], VecDeque<VecDeque<u8>>) {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let image = image
        .lines()
        .map(|line| line.bytes().collect::<VecDeque<_>>())
        .collect::<VecDeque<_>>();

    (algo.as_bytes(), image)
}

fn padding(image: &mut VecDeque<VecDeque<u8>>, default: u8) {
    let line_len = image[0].len();
    image.push_front(VecDeque::from(vec![default; line_len]));
    image.push_front(VecDeque::from(vec![default; line_len]));
    image.push_back(VecDeque::from(vec![default; line_len]));
    image.push_back(VecDeque::from(vec![default; line_len]));

    for line in image {
        line.push_back(default);
        line.push_back(default);
        line.push_front(default);
        line.push_front(default);
    }
}

fn trim(image: &mut VecDeque<VecDeque<u8>>) {
    image.pop_back();
    image.pop_front();
    image.iter_mut().for_each(|line| {
        line.pop_back();
        line.pop_front();
    });
}

fn enhance(image: &mut VecDeque<VecDeque<u8>>, algo: &[u8], default: u8) {
    padding(image, default);
    let mut output = image.clone();

    for ys in 0..image.len() {
        for xs in 0..image[0].len() {
            let n = [
                (ys.checked_sub(1)
                    .and_then(|y| xs.checked_sub(1).map(|x| image[y][x]))),
                ys.checked_sub(1).map(|y| image[y][xs]),
                ys.checked_sub(1)
                    .and_then(|y| image[y].get(xs + 1))
                    .copied(),
                xs.checked_sub(1).map(|x| image[ys][x]),
                Some(image[ys][xs]),
                image[ys].get(xs + 1).copied(),
                image
                    .get(ys + 1)
                    .and_then(|line| xs.checked_sub(1).map(|x| line[x])),
                image.get(ys + 1).map(|line| line[xs]),
                image.get(ys + 1).and_then(|line| line.get(xs + 1)).copied(),
            ]
            .into_iter()
            .rev()
            .enumerate()
            .filter(|&(_, b)| b == Some(b'#'))
            .fold(0, |acc, (i, _)| acc + (1 << i));
            output[ys][xs] = algo[n];
        }
    }

    trim(&mut output);
    *image = output;
}

fn day20a(image: &mut VecDeque<VecDeque<u8>>, algo: &[u8]) -> usize {
    enhance(image, algo, b'.');
    enhance(image, algo, b'#');
    image.iter().flatten().filter(|&&b| b == b'#').count()
}

fn day20b(image: &mut VecDeque<VecDeque<u8>>, algo: &[u8]) -> usize {
    for _ in 0..24 {
        enhance(image, algo, b'.');
        enhance(image, algo, b'#');
    }
    image.iter().flatten().filter(|&&b| b == b'#').count()
}

fn main() {
    let input = include_str!("../inputs/day20.txt");
    let (algo, mut image) = parse_input(input);

    let day20a = day20a(&mut image, algo);
    assert_eq!(day20a, 5503);
    println!("day20a: {}", day20a);

    let day20b = day20b(&mut image, algo);
    assert_eq!(day20b, 19156);
    println!("day20b: {}", day20b);
}
