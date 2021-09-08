#[derive(Debug)]
enum Action {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

#[derive(Debug)]
enum Direction {
    NORTH, //0
    SOUTH, //2
    EAST,  //1
    WEST,  //3
}

#[derive(Debug)]
struct Ferry {
    direction: Direction,
    position: (i32, i32),
    waypoint_position: (i32, i32),
}

impl Ferry {
    fn new() -> Self {
        Self {
            direction: Direction::EAST,
            position: (0, 0),
            waypoint_position: (-1, 10),
        }
    }

    fn execute_instruction_2(&mut self, (action, value): &(Action, i32)) {

        match action {
            Action::NORTH => self.waypoint_position.0 -= value,
            Action::SOUTH => self.waypoint_position.0 += value,
            Action::EAST => self.waypoint_position.1 += value,
            Action::WEST => self.waypoint_position.1 -= value,
            Action::FORWARD => {
                self.position.0 += value * self.waypoint_position.0;
                self.position.1 += value * self.waypoint_position.1
            }
            Action::LEFT => {
                for _ in 0..value / 90 % 4 {
                    let tmp = self.waypoint_position.0;
                    self.waypoint_position.0 = -self.waypoint_position.1;
                    self.waypoint_position.1 = tmp;
                }
            }
            Action::RIGHT => {
                for _ in 0..value / 90 % 4 {
                    let tmp = self.waypoint_position.1;
                    self.waypoint_position.1 = -self.waypoint_position.0;
                    self.waypoint_position.0 = tmp;
                }
            }
        }
    }

    fn execute_instruction(&mut self, (action, value): &(Action, i32)) {
        match action {
            Action::NORTH => self.position.0 -= value,
            Action::SOUTH => self.position.0 += value,
            Action::EAST => self.position.1 += value,
            Action::WEST => self.position.1 -= value,
            Action::FORWARD => match self.direction {
                Direction::NORTH => self.position.0 -= value,
                Direction::SOUTH => self.position.0 += value,
                Direction::EAST => self.position.1 += value,
                Direction::WEST => self.position.1 -= value,
            },
            Action::LEFT => self.set_direction_from_number(
                match value / 90 % 4 {
                    0 => self.get_direction_as_number(),
                    1 => self.get_direction_as_number() + 3,
                    2 => self.get_direction_as_number() + 2,
                    3 => self.get_direction_as_number() + 1,
                    _ => panic!("Invalid direction change value"),
                } % 4,
            ),
            Action::RIGHT => self.set_direction_from_number(
                match value / 90 % 4 {
                    0 => self.get_direction_as_number(),
                    1 => self.get_direction_as_number() + 1,
                    2 => self.get_direction_as_number() + 2,
                    3 => self.get_direction_as_number() + 3,
                    _ => panic!("Invalid direction change value"),
                } % 4,
            ),
        }
    }

    fn get_direction_as_number(&self) -> i32 {
        match self.direction {
            Direction::NORTH => 0,
            Direction::SOUTH => 2,
            Direction::EAST => 1,
            Direction::WEST => 3,
        }
    }

    fn set_direction_from_number(&mut self, direction: i32) {
        self.direction = match direction {
            0 => Direction::NORTH,
            1 => Direction::EAST,
            2 => Direction::SOUTH,
            3 => Direction::WEST,
            _ => panic!("Invalid direction code!"),
        }
    }

    fn get_instr_from_str(instr: &str) -> Vec<(Action, i32)> {
        instr
            .lines()
            .map(|line| line.split_at(1))
            .map(|(action, value)| {
                (
                    match action {
                        "N" => Action::NORTH,
                        "S" => Action::SOUTH,
                        "E" => Action::EAST,
                        "W" => Action::WEST,
                        "L" => Action::LEFT,
                        "R" => Action::RIGHT,
                        "F" => Action::FORWARD,
                        _ => panic!("Error while parsing instruction!"),
                    },
                    value.parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    }

    fn run_program(&mut self, program: &Vec<(Action, i32)>) {
        for instruction in program {
            self.execute_instruction(instruction);
        }
    }
    fn run_program_2(&mut self, program: &Vec<(Action, i32)>) {
        for instruction in program {
            self.execute_instruction_2(instruction);
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions = Ferry::get_instr_from_str(&input);
    let mut ferry = Ferry::new();
    ferry.run_program(&instructions);
    println!(
        "Part 1: {}",
        ferry.position.0.abs() + ferry.position.1.abs()
    );
    ferry = Ferry::new();
    ferry.run_program_2(&instructions);
    println!(
        "Part 2: {}",
        ferry.position.0.abs() + ferry.position.1.abs()
    );
}
