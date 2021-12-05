#[derive(Debug, Default)]
struct Number {
    number: u32,
    marked: bool,
}

impl Number {
    fn new(number: u32) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

#[derive(Debug, Default)]
struct Board {
    grid: [Number; 25],
    won: bool,
}

impl Board {
    fn mark_number(&mut self, n: u32) {
        if let Some((pos, number)) = self
            .grid
            .iter_mut()
            .enumerate()
            .find(|(_, number)| number.number == n)
        {
            number.marked = true;
            self.check_winner(pos);
        }
    }

    fn check_winner(&mut self, pos: usize) {
        // row
        if self
            .grid
            .chunks_exact(5)
            .nth(pos / 5)
            .unwrap()
            .iter()
            .all(|n| n.marked)
            // column
            || self.grid.iter().skip(pos % 5).step_by(5).all(|n| n.marked)
        {
            self.won = true;
        }
    }

    fn sum_unmarked(&self) -> u32 {
        self.grid
            .iter()
            .filter(|n| !n.marked)
            .map(|n| n.number)
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<Board>, Vec<u32>) {
    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let numbers = numbers.split(',').map(|s| s.parse().unwrap()).collect();
    let boards = boards
        .split("\n\n")
        .map(|b| {
            let mut board = Board::default();
            for (s, n) in b.split_ascii_whitespace().zip(&mut board.grid) {
                *n = Number::new(s.parse().unwrap());
            }
            board
        })
        .collect();

    (boards, numbers)
}

fn play_game(boards: &mut [Board], numbers: &[u32]) -> (u32, u32) {
    let mut score_first_winner = 0;
    let mut score_last_winner = 0;

    for &number in numbers {
        for board in boards.iter_mut().filter(|b| !b.won) {
            board.mark_number(number);
            if board.won {
                let score = board.sum_unmarked() * number;
                if score_first_winner == 0 {
                    score_first_winner = score;
                }
                score_last_winner = score;
            }
        }
    }

    (score_first_winner, score_last_winner)
}

fn main() {
    let input = include_str!("../../inputs/day4.txt");
    let (mut boards, numbers) = parse_input(input);
    let (day4a, day4b) = play_game(&mut boards, &numbers);

    debug_assert_eq!(day4a, 29440);
    println!("First winner score: {}", day4a);

    debug_assert_eq!(day4b, 13884);
    println!("Last winner score: {}", day4b);
}
