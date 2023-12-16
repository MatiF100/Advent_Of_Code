use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let split = line.split_once(": ").unwrap();
            (
                split
                    .0
                    .trim()
                    .split_once(" ")
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
                split
                    .1
                    .trim()
                    .split("; ")
                    .map(|subset| {
                        subset
                            .split(", ")
                            .map(|val| {
                                let balls = val.trim().split_once(" ").unwrap();
                                (balls.0.parse::<usize>().unwrap(), balls.1)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let part1: usize = input
        .iter()
        .filter(|(_, subsets)| {
            subsets.iter().fold(true, |acc, val| {
                acc && val.iter().fold(acc, |acc, v| {
                    acc && if v.1 == "red" {
                        v.0 <= 12
                    } else if v.1 == "green" {
                        v.0 <= 13
                    } else if v.1 == "blue" {
                        v.0 <= 14
                    } else {
                        true
                    }
                })
            })
        })
        .map(|(i, _)| i)
        .sum();

        
    let part2: usize = input
        .iter()
        .map(|(_, subsets)| {
            subsets
                .iter()
                .fold(HashMap::new(), |mut acc, val| {
                    val.iter().for_each(|v| {
                        acc.entry(v.1).and_modify(|st| if *st < v.0 {*st = v.0}).or_insert(v.0);
                    });
                    acc
                })
                .iter()
                .fold(1, |acc, (_,v)| acc * v)
                
        })
        .sum();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
