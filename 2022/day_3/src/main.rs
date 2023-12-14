fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    let input = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| (a.as_bytes(), b.as_bytes()))
        .collect::<Vec<_>>();

    let part1: usize = input
        .iter()
        .map(|(a, b)| a.iter().filter(|x| b.contains(x)).next().unwrap())
        .map(|x| if x >= &97 { x - 96 } else { x - 65 + 27 } as usize)
        .sum();

    let part2: usize = input
        .windows(3)
        .step_by(3)
        .map(|x| x.iter().map(|x| [x.0, x.1].concat()))
        .map(|mut x| {
            let (a, b, c) = {
                (x.next().unwrap(), x.next().unwrap(), x.next().unwrap())
            };
            a.iter()
                .filter(|p| b.iter().filter(|q| c.contains(q)).any(|v| v == *p))
                .next()
                .unwrap()
                .to_owned()
        })
        .map(|x| if x >= 97 { x - 96 } else { x - 65 + 27 } as usize)
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
