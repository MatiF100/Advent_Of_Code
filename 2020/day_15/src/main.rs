use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    state: HashMap<usize, usize>,
    last_move: usize,
    turn: usize,
}

impl Game {
    fn new(initial_moves: &Vec<usize>) -> Self {
        let mut ret = HashMap::new();
        for (idx, next_move) in initial_moves.iter().enumerate() {
            ret.insert(*next_move, idx);
        }
        ret.remove(&initial_moves.last().unwrap());
        Self {
            turn: ret.values().max().unwrap() + 1,
            last_move: *initial_moves.last().unwrap(),
            state: ret,
        }
    }

    fn perform_step(&mut self) {
        let age = match self.state.get(&self.last_move) {
            Some(birth) => self.turn - birth,
            None => 0,
        };
        self.state.insert(self.last_move, self.turn);
        self.turn += 1;
        self.last_move = age;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut game = Game::new(&input);
    while game.turn != 2020 - 1 {
        game.perform_step();
    }
    println!("Part 1: {}", game.last_move);

    game = Game::new(&input);
    while game.turn != 30000000 - 1 {
        game.perform_step();
    }
    println!("Part 2: {}", game.last_move);
}
