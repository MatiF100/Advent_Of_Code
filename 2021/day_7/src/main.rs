fn main() {
    let input = if let Ok(s) = std::fs::read_to_string("input.txt") {
        s
    } else {
        "16,1,2,0,4,2,7,1,2,14".to_owned()
    };
    let input = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<i32>) -> i32 {
    let max = *input.iter().max().unwrap();
    let min = *input.iter().min().unwrap();
    let mut best = i32::MAX;
    for i in min..max {
        let sum = input.iter().map(|x| (*x - i).abs()).sum::<i32>();
        if sum < best {
            best = sum;
        }
    }
    best
}
fn part2(input: &Vec<i32>) -> i32 {
    let max = *input.iter().max().unwrap();
    let min = *input.iter().min().unwrap();
    let mut best = i32::MAX;
    for i in min..max {
        let sum = input
            .iter()
            .map(|x| (1..=(*x - i).abs()).sum::<i32>())
            .sum::<i32>();
        if sum < best {
            best = sum;
        }
    }
    best
}
