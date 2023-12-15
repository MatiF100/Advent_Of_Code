fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let part1: u32 = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut num| num.clone().last().unwrap() + num.next().unwrap() * 10)
        .sum();

    println!("Part 1: {}", part1);

    let dict = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let part2: u32 = input
        .lines()
        .map(|line| {
            (
                dict.iter()
                    .filter_map(|pat| {
                        if let Some(idx) = line.find(pat.0) {
                            Some((idx, pat.1))
                        } else {
                           if let Some (idx) = line.find(|c: char| c.is_numeric()){Some((idx, (line.as_bytes()[idx] as char).to_digit(10).unwrap()))} else {None}
                        }
                    })
                    .min_by(|a, b| a.0.cmp(&b.0))
                    .unwrap()
                    .1,
                dict.iter()
                    .filter_map(|pat| {
                        if let Some(idx) = line.rfind(pat.0) {
                            Some((idx, pat.1))
                        } else {
                           if let Some (idx) = line.rfind(|c: char| c.is_numeric()){Some((idx, (line.as_bytes()[idx] as char).to_digit(10).unwrap()))} else {None}
                        }
                    })
                    .max_by(|a, b| a.0.cmp(&b.0))
                    .unwrap()
                    .1,
            )
        })
        .map(|num| num.0 * 10 + num.1)
        .sum();
    println!("Part 2: {}", part2);
}
