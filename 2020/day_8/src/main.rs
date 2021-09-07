use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq, Eq)]
enum Opcode {
    NOP,
    ACC,
    JMP,

    ERR,
}

#[derive(Clone, Debug)]
struct Machine {
    program: Vec<(Opcode, i32)>,
    accumulator: i32,
    program_counter: i32,
}

impl Machine {
    fn new(program: Vec<(Opcode, i32)>) -> Self {
        Self {
            program: program,
            accumulator: 0,
            program_counter: 0,
        }
    }

    fn execute_instruction(&mut self) {
        assert!(self.program_counter >= 0);
        match self.program.get(self.program_counter as usize) {
            Some((opcode, argument)) => match opcode {
                Opcode::NOP => {
                    self.program_counter += 1;
                }
                Opcode::ACC => {
                    self.accumulator += *argument;
                    self.program_counter += 1;
                }
                Opcode::JMP => {
                    self.program_counter += *argument;
                }
                Opcode::ERR => {
                    panic!("Encountered unknown opcode... Aborting now...");
                }
            },
            None => panic!(
                "Accumulator outside of program area. Ermmmm... Segmentation fault... or something"
            ),
        }
    }
    fn get_op_from_str(op: &str) -> Opcode {
        match op {
            "nop" => Opcode::NOP,
            "acc" => Opcode::ACC,
            "jmp" => Opcode::JMP,
            _ => Opcode::ERR,
        }
    }

    fn run_until_loop(&mut self) -> i32{
        let mut memory: HashSet<i32> = HashSet::new();
        while !memory.contains(&self.program_counter){
            memory.insert(self.program_counter);
            self.execute_instruction();
            if self.program_counter >= self.program.len() as i32{
                break
            }
        }

        self.accumulator
    }

    fn run_until_loop_or_finish(&mut self) -> (i32, i32, bool){
        let mut memory: HashSet<i32> = HashSet::new();
        let mut finish_flag = false;
        while !memory.contains(&self.program_counter){
            memory.insert(self.program_counter);
            self.execute_instruction();
            if self.program_counter >= self.program.len() as i32{
                finish_flag = true;
                break
            }
        }

        (self.accumulator, self.program_counter, finish_flag)
    }

    fn run_and_fix(&self) -> i32{
        for indexer in 0..self.program.len(){
            let mut machine_copy = self.clone();
            let status = match self.program[indexer]{
                (Opcode::ACC, _) => continue,
                (Opcode::ERR, _) => panic!("Unknown opcode encountered! Aborting now..."),
                (Opcode::JMP, arg) => {
                    machine_copy.program[indexer] = (Opcode::NOP, arg);
                    machine_copy.run_until_loop_or_finish()
                },
                (Opcode::NOP, arg) => {
                    machine_copy.program[indexer] = (Opcode::JMP, arg);
                    machine_copy.run_until_loop_or_finish()
                },
            };

            if status.2 == true{
                return machine_copy.accumulator;
            }
        }
        -1
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(op, arg)| (Machine::get_op_from_str(op), arg.parse::<i32>().unwrap()))
        .collect::<Vec<(Opcode, i32)>>();
    let machine = Machine::new(input);
    println!("Part 1: {}", machine.clone().run_until_loop());
    println!("Part 2: {}", machine.clone().run_and_fix());
}
