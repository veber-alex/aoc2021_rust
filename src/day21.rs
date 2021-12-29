#[derive(Debug)]
struct Player {
    score: u16,
    position: u16,
}

impl Player {
    fn new(position: u16) -> Self {
        Self { score: 0, position }
    }

    fn play(&mut self, roll: u16) {
        self.position = (self.position + roll) % 10;
        if self.position == 0 {
            self.position = 10;
        }
        self.score += self.position;
    }
}

fn day21a(p1: Player, p2: Player) -> usize {
    let mut dice = (1..101).cycle();
    let mut turn = 0;
    let mut players = [p1, p2];

    loop {
        let p = &mut players[turn % 2];
        p.play(dice.by_ref().take(3).sum());
        if p.score >= 1000 {
            break players[!turn % 2].score as usize * (turn * 3 + 3);
        }
        turn += 1;
    }
}

fn day21b(p1: Player, p2: Player, roll: u16) {
    let mut turn = false;
    let mut players = [p1, p2];

    loop {
        let p = &mut players[turn as usize];
        p.play(roll);
        if p.score >= 21 {
            break;
        }
        turn = !turn;
    }
}

fn parse_input(input: &str) -> (u16, u16) {
    let mut iter = input
        .lines()
        .map(|line| line.rsplit_once(' ').unwrap().1.parse().unwrap());

    (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
    let input = include_str!("../inputs/day21.txt");
    let (p1, p2) = parse_input(input);

    let day21a = day21a(Player::new(p1), Player::new(p2));
    assert_eq!(day21a, 711480);
    println!("day21a: {}", day21a);
}
