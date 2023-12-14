use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let s = line.split_once(",").unwrap();
            (s.0.split_once("-").unwrap(), s.1.split_once("-").unwrap())
        })
        .map(|(a, b)| {
            (
                (a.0.parse::<usize>().unwrap()..=a.1.parse::<usize>().unwrap())
                    .step_by(1)
                    .collect::<HashSet<_>>(),
                (b.0.parse::<usize>().unwrap()..=b.1.parse::<usize>().unwrap())
                    .step_by(1)
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<Vec<_>>();


    let part1 = input.iter().filter(|(a, b)| a.is_subset(b) || a.is_superset(b)).count();
    let part2 = input.iter().filter(|(a, b)| a.intersection(b).count() > 0).count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
