struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn submerge_by(&mut self, y: i32) {
        self.y += y;
    }

    fn forward_by(&mut self, x: i32) {
        self.x += x;
    }

    fn reaim_by(&mut self, aim: i32) {
        self.aim += aim;
    }

    fn simple_process_move(&mut self, dir: &str, val: i32) {
        match dir {
            "forward" => self.forward_by(val),
            "down" => self.submerge_by(val),
            "up" => self.submerge_by(-val),
            _ => panic!("Unknown move! Aborting!"),
        }
    }
    fn process_move(&mut self, dir: &str, val: i32) {
        match dir {
            "forward" => {
                self.forward_by(val);
                self.submerge_by(self.aim * val)
            }
            "down" => self.reaim_by(val),
            "up" => self.reaim_by(-val),
            _ => panic!("Unknown move! Aborting!"),
        }
    }
}

fn main() {
    let input = match std::fs::read_to_string("input.txt") {
        Ok(s) => s,
        Err(_) => "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            .to_owned(),
    };
    let input = input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(d, v)| (d, v.parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<(&str, i32)>) -> i32 {
    let mut submarine = Submarine::new();
    for instr in input {
        submarine.simple_process_move(instr.0, instr.1);
    }
    submarine.x * submarine.y
}

fn part2(input: &Vec<(&str, i32)>) -> i32 {
    let mut submarine = Submarine::new();
    for instr in input {
        submarine.process_move(instr.0, instr.1);
    }
    submarine.x * submarine.y
}
