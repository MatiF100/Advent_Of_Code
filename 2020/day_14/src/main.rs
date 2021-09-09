use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MaskField {
    ONE,
    ZERO,
    UNSPECIFIED,
}

#[derive(Debug)]
enum Instruction {
    MASK([MaskField; 36]),
    MEM((usize, u64)),
}

#[derive(Debug)]
struct Computer {
    memory: HashMap<usize, u64>,
    mask: [MaskField; 36],
}

impl Computer {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            mask: [MaskField::UNSPECIFIED; 36],
        }
    }

    fn execute_instruction(&mut self, instr: &Instruction) {
        match *instr {
            Instruction::MASK(mask) => self.mask = mask,
            Instruction::MEM((target, value)) => {
                self.memory.insert(target, self.apply_mask(value));
            }
        }
    }

    fn execute_instruction_2(&mut self, instr: &Instruction) {
        match *instr {
            Instruction::MASK(mask) => self.mask = mask,
            Instruction::MEM((target, value)) => {
                let addresses = self.apply_mask_2(target);
                for address in addresses {
                    self.memory.insert(address, value);
                }
            }
        }
    }

    fn parse_instruction(instr: &str) -> Instruction {
        instr
            .split_once(" = ")
            .and_then(|(target, value)| match target {
                "mask" => Some(Instruction::MASK(Self::parse_mask(value))),
                mem => mem
                    .split_once("[")
                    .and_then(|(_, address)| address.strip_suffix("]"))
                    .and_then(|address| {
                        Some(Instruction::MEM((
                            address.parse::<usize>().unwrap(),
                            value.parse::<u64>().unwrap(),
                        )))
                    }),
            })
            .expect(&format!(
                "Error while parsing instruction: {:?}, Aborting now...",
                instr
            ))
    }

    fn apply_mask(&self, value: u64) -> u64 {
        let or_mask: u64 = self
            .mask
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, field)| match field {
                MaskField::ONE => 2u64.pow(idx as u32),
                _ => 0,
            })
            .sum();
        let and_mask: u64 = self
            .mask
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, field)| match field {
                MaskField::ZERO => 0,
                _ => 2u64.pow(idx as u32),
            })
            .sum();

        (value | or_mask) & and_mask
    }

    fn apply_mask_2(&self, address: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = vec![0];

        for (idx, field) in self.mask.iter().rev().enumerate() {
            let mut alt_ret: Vec<usize> = Vec::new();
            match field {
                MaskField::ZERO => {
                    for possible_addr in ret.iter_mut() {
                        *possible_addr += 2usize.pow(idx as u32) * ((address >> idx) & 1)
                    }
                }
                MaskField::ONE => {
                    for possible_addr in ret.iter_mut() {
                        *possible_addr += 2usize.pow(idx as u32)
                    }
                }
                MaskField::UNSPECIFIED => {
                    for possible_addr in ret.iter_mut() {
                        alt_ret.push(*possible_addr);
                        *possible_addr += 2usize.pow(idx as u32)
                    }
                }
            };
            ret.append(&mut alt_ret);
        }
        ret
    }

    fn parse_mask(mask: &str) -> [MaskField; 36] {
        let mut ret = [MaskField::UNSPECIFIED; 36];
        for (idx, field) in mask.chars().enumerate() {
            ret[idx] = match field {
                '0' => MaskField::ZERO,
                '1' => MaskField::ONE,
                'X' => MaskField::UNSPECIFIED,
                _ => panic!("Error while parsing mask {}", mask),
            };
        }
        ret
    }

    fn sum_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .lines()
        .map(|line| Computer::parse_instruction(line))
        .collect::<Vec<_>>();
    let mut computer = Computer::new();
    for instruction in &input {
        computer.execute_instruction(instruction);
    }
    println!("Part 1: {}", computer.sum_memory());
    computer = Computer::new();
    for instruction in input {
        computer.execute_instruction_2(&instruction);
    }
    println!("Part 2: {}", computer.sum_memory());
}
