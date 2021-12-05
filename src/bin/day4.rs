#[derive(Debug, Default)]
struct Board {
    grid: [i32; 25],
    won: bool,
}

impl Board {
    fn mark_number(&mut self, n: i32) {
        if let Some((pos, number)) = self
            .grid
            .iter_mut()
            .enumerate()
            .find(|(_, number)| **number == n)
        {
            *number = -1;
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
            .all(|n| *n < 0)
            // column
            || self.grid.iter().skip(pos % 5).step_by(5).all(|n| *n < 0)
        {
            self.won = true;
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.grid.iter().filter(|n| **n > 0).sum()
    }
}

fn parse_input(input: &str) -> (Vec<Board>, Vec<i32>) {
    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let numbers = numbers.split(',').map(|s| s.parse().unwrap()).collect();
    let boards = boards
        .split("\n\n")
        .map(|b| {
            let mut board = Board::default();
            for (s, n) in b.split_ascii_whitespace().zip(&mut board.grid) {
                *n = s.parse().unwrap();
            }
            board
        })
        .collect();

    (boards, numbers)
}

fn play_game(boards: &mut [Board], numbers: &[i32]) -> (i32, i32) {
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
