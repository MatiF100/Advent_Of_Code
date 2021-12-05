use std::collections::HashSet;
fn main() {
    let input = if let Ok(s) = std::fs::read_to_string("input.txt") {
        s
    } else {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .to_owned()
    };

    let input = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| (a.split_once(',').unwrap(), b.split_once(',').unwrap()))
        .map(|((x1, y1), (x2, y2))| {
            (
                (x1.parse::<i32>().unwrap(), y1.parse::<i32>().unwrap()),
                (x2.parse::<i32>().unwrap(), y2.parse::<i32>().unwrap()),
            )
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part2(input: &Vec<((i32, i32), (i32, i32))>) -> usize {
    let mut points: Vec<(i32, i32)> = Vec::new();
    for pair in input {
        if pair.0 .0 == pair.1 .0 || pair.0 .1 == pair.1 .1 {
            let (x_1, x_2) = if !(pair.0 .0 >= pair.1 .0) {
                (pair.0 .0, pair.1 .0)
            } else {
                (pair.1 .0, pair.0 .0)
            };
            let (y_1, y_2) = if !(pair.0 .1 >= pair.1 .1) {
                (pair.0 .1, pair.1 .1)
            } else {
                (pair.1 .1, pair.0 .1)
            };
            for x in x_1..=x_2 {
                for y in y_1..=y_2 {
                    points.push((x, y));
                }
            }
        } else {
            let step_x = if !(pair.0 .0 >= pair.1 .0) { 1 } else { -1 };
            let step_y = if !(pair.0 .1 >= pair.1 .1) { 1 } else { -1 };
            let mut x = pair.0 .0;
            let mut y = pair.0 .1;

            while x != pair.1 .0 + step_x && y != pair.1 .1 + step_y {
                points.push((x, y));
                x += step_x;
                y += step_y;
            }
        }
    }
    let mut used: HashSet<(i32, i32)> = HashSet::new();
    for point in &points {
        let mut cnt = 0;
        for refer in &points {
            if point == refer {
                cnt += 1;
            }
        }
        if cnt >= 2 {
            used.insert(*point);
        }
    }
    used.len()
}

fn part1(input: &Vec<((i32, i32), (i32, i32))>) -> usize {
    let mut points: Vec<(i32, i32)> = Vec::new();
    for pair in input {
        if pair.0 .0 == pair.1 .0 || pair.0 .1 == pair.1 .1 {
            let (x_1, x_2) = if !(pair.0 .0 >= pair.1 .0) {
                (pair.0 .0, pair.1 .0)
            } else {
                (pair.1 .0, pair.0 .0)
            };
            let (y_1, y_2) = if !(pair.0 .1 >= pair.1 .1) {
                (pair.0 .1, pair.1 .1)
            } else {
                (pair.1 .1, pair.0 .1)
            };
            for x in x_1..=x_2 {
                for y in y_1..=y_2 {
                    points.push((x, y));
                }
            }
        }
    }
    let mut used: HashSet<(i32, i32)> = HashSet::new();
    for point in &points {
        let mut cnt = 0;
        for refer in &points {
            if point == refer {
                cnt += 1;
            }
        }
        if cnt >= 2 {
            used.insert(*point);
        }
    }
    used.len()
}
