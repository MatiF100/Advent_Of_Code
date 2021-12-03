fn main() {
    let input = match std::fs::read_to_string("input.txt") {
        Ok(s) => s,
        Err(_) => "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .to_owned(),
    };

    let size = input.lines().next().unwrap().trim().len();

    let input = input
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", part1(&input, size));
    println!("Part 2: {}", part2(&input, size));
}

fn part2(input: &Vec<i32>, line_size: usize) -> i32 {
    get_dioxide_rating(input, line_size) * get_oxygen_rating(input, line_size)
}

fn get_dioxide_rating(input: &Vec<i32>, line_size: usize) -> i32 {
    let mut co_vec = input.clone();
    let mut mask = (line_size - 1) as i32;
    while co_vec.len() > 1 {
        let res = determine_count(&co_vec, line_size);
        let res = res
            .iter()
            .map(|(zero, one)| zero > one)
            .collect::<Vec<bool>>();
        let mut tmp_vec: Vec<i32> = Vec::new();
        for word in &co_vec {
            if ((*word & 2i32.pow(mask as u32)) != 0) == res[mask as usize] {
                tmp_vec.push(*word);
            }
        }
        co_vec = tmp_vec;
        mask -= 1;
    }
    co_vec[0]
}

fn get_oxygen_rating(input: &Vec<i32>, line_size: usize) -> i32 {
    let mut ox_vec = input.clone();
    let mut mask = (line_size - 1) as i32;
    while ox_vec.len() > 1 {
        let res = determine_count(&ox_vec, line_size);
        let res = res
            .iter()
            .map(|(zero, one)| zero <= one)
            .collect::<Vec<bool>>();
        let mut tmp_vec: Vec<i32> = Vec::new();
        for word in &ox_vec {
            if ((*word & 2i32.pow(mask as u32)) != 0) == res[mask as usize] {
                tmp_vec.push(*word);
            }
        }
        ox_vec = tmp_vec;
        mask -= 1;
    }
    ox_vec[0]
}

fn determine_count(input: &Vec<i32>, line_size: usize) -> Vec<(i32, i32)> {
    let mut mask = 0;
    let mut res: Vec<(i32, i32)> = Vec::new();
    while mask as usize <= line_size - 1 {
        res.push((0, 0));
        for word in input {
            if (word & 2i32.pow(mask as u32)) == 0 {
                res[mask as usize].0 += 1;
            } else {
                res[mask as usize].1 += 1;
            }
        }
        mask += 1;
    }
    res
}

fn part1(input: &Vec<i32>, line_size: usize) -> i32 {
    let mut mask: i32 = 0;
    let mut res: Vec<(i32, i32)> = Vec::new();
    while mask as usize <= line_size - 1 {
        res.push((0, 0));
        for word in input {
            if (word & 2i32.pow(mask as u32)) == 0 {
                res[mask as usize].0 += 1;
            } else {
                res[mask as usize].1 += 1;
            }
        }
        mask += 1;
    }
    let (mut gamma, mut epsilon) = (0, 0);

    res.iter()
        .map(|(zero, one)| zero < one)
        .enumerate()
        .map(|(c, ge)| {
            if ge {
                gamma += 2i32.pow(c as u32)
            } else {
                epsilon += 2i32.pow(c as u32)
            }
        })
        .for_each(drop);

    gamma * epsilon
}
