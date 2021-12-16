use Symbol::*;

#[derive(Debug)]
enum Symbol {
    Literal(u64),
    Instruction { kind: u8, count: u8 },
}

#[derive(Debug)]
struct Parser<'a> {
    bytes: &'a [u8],
    scratch: Vec<u8>,
    version_sum: u16,
    code: Vec<Symbol>,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            scratch: vec![],
            version_sum: 0,
            code: vec![],
        }
    }

    fn take(&mut self, n: usize) -> &'a [u8] {
        let bytes = &self.bytes[..n];
        self.bytes = &self.bytes[n..];
        bytes
    }

    fn one(&mut self) -> u8 {
        let byte = self.bytes[0];
        self.bytes = &self.bytes[1..];
        byte
    }

    fn parse_packet(&mut self) {
        let version = to_int(self.take(3)) as u8;
        self.version_sum += version as u16;
        let kind = to_int(self.take(3)) as u8;

        match kind {
            4 => self.parse_literal(),
            _ => {
                let len_type_id = self.one();
                match len_type_id {
                    0 => self.parse_operator_0(kind),
                    1 => self.parse_operator_1(kind),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn parse_operator_1(&mut self, kind: u8) {
        let count = to_int(self.take(11)) as u8;
        for _ in 0..count {
            self.parse_packet();
        }

        self.code.push(Instruction { kind, count })
    }

    fn parse_operator_0(&mut self, kind: u8) {
        let mut count = 0;
        let end = to_int(self.take(15)) as usize + self.bytes.as_ptr() as usize;

        while self.bytes.as_ptr() as usize != end {
            self.parse_packet();
            count += 1;
        }

        self.code.push(Instruction { kind, count })
    }

    fn parse_literal(&mut self) {
        loop {
            let key = self.one();
            let bytes = self.take(4);
            self.scratch.extend_from_slice(bytes);
            if key == 0 {
                self.code.push(Literal(to_int(&self.scratch)));
                self.scratch.clear();
                break;
            }
        }
    }

    fn run(&self) -> u64 {
        let mut stack = vec![];

        for sym in &self.code {
            match *sym {
                Literal(v) => stack.push(v),
                Instruction { kind, count } => {
                    let n = match kind {
                        0 => (0..count).map(|_| stack.pop().unwrap()).sum(),
                        1 => (0..count).map(|_| stack.pop().unwrap()).product(),
                        2 => (0..count).map(|_| stack.pop().unwrap()).min().unwrap(),
                        3 => (0..count).map(|_| stack.pop().unwrap()).max().unwrap(),
                        5 => (stack.pop().unwrap() < stack.pop().unwrap()) as _,
                        6 => (stack.pop().unwrap() > stack.pop().unwrap()) as _,
                        7 => (stack.pop().unwrap() == stack.pop().unwrap()) as _,
                        _ => unreachable!(),
                    };
                    stack.push(n);
                }
            }
        }

        stack[0]
    }
}

fn to_int(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(i, &b)| (b == 1).then(|| 1 << i))
        .sum()
}

fn parse_input(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .flat_map(|&b| match b {
            b'0' => [0, 0, 0, 0],
            b'1' => [0, 0, 0, 1],
            b'2' => [0, 0, 1, 0],
            b'3' => [0, 0, 1, 1],
            b'4' => [0, 1, 0, 0],
            b'5' => [0, 1, 0, 1],
            b'6' => [0, 1, 1, 0],
            b'7' => [0, 1, 1, 1],
            b'8' => [1, 0, 0, 0],
            b'9' => [1, 0, 0, 1],
            b'A' => [1, 0, 1, 0],
            b'B' => [1, 0, 1, 1],
            b'C' => [1, 1, 0, 0],
            b'D' => [1, 1, 0, 1],
            b'E' => [1, 1, 1, 0],
            b'F' => [1, 1, 1, 1],
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let input = include_bytes!("../inputs/day16.txt");
    let bytes = parse_input(input);
    let mut parser = Parser::new(&bytes);
    parser.parse_packet();

    let day16a = parser.version_sum;
    assert_eq!(day16a, 943);
    println!("day16a: {}", day16a);

    let day16b = parser.run();
    assert_eq!(day16b, 167737115857);
    println!("day16b: {}", day16b);
}
