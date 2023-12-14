fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    let input = input
        .lines()
        .map(|line| match line.split_once(" ").unwrap() {
            ("A", "X") => (1, 1),
            ("A", "Y") => (1, 2),
            ("A", "Z") => (1, 3),
            ("B", "X") => (2, 1),
            ("B", "Y") => (2, 2),
            ("B", "Z") => (2, 3),
            ("C", "X") => (3, 1),
            ("C", "Y") => (3, 2),
            ("C", "Z") => (3, 3),
            _ => (0, 0),
        })
        .collect::<Vec<_>>();

    let part1 = input.iter().fold(0, |acc, x| {
        acc + match x {
            //Draw
            (a, b) if a == b => 3 + x.1,
            //Win
            (a, b) if a - b == -1 || a - b == 2 => 6 + x.1,
            //Lose
            (a, b) if b - a == 2 || b - a == -1 => x.1,
            _ => 0,
        }
    });

    let part2 = input.iter().fold(0, |acc, x| {
        acc + match x {
            //Lose
            (a, 1) => (1..=3).filter(|b| b - a == 2 || b - a == -1).next().unwrap_or(0),
            //Draw
            (a, 2) => (1..=3).filter(|b| a == b).next().unwrap_or(0) + 3,
            //Win
            (a, 3) => (1..=3).filter(|b| a - b == 2 || a - b == -1).next().unwrap_or(0) + 6,
            _ => 0,
        }
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
