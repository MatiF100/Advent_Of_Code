fn main(){
    let input = if let Ok(s) = std::fs::read_to_string("input.txt") {
        s
    } else {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\r
\r
22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19\r
\r
3 15  0  2 22\r
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6\r
\r
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"
        .to_owned()
    };

    let mut iter = input.split("\r\n\r\n");
    let numbers = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let boards = iter
        .map(|bs| {
            bs.lines()
                .map(|line| {
                    line.trim()
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&numbers, &boards));
    println!("Part 2: {}", part2(&numbers, &boards));
}
fn part2(numbers: &Vec<i32>, boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut draw: Vec<i32> = Vec::new();
    let mut drawer = numbers.iter();
    let mut winning: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut winning_no = 0;
    while draw.len() != numbers.len() {
        draw.push(*drawer.next().unwrap());
        for board in boards {
            for row in board {
                if check_for_match(&draw, &row) && !winning.contains(board) {
                    winning.push(board.clone());
                    winning_no = *draw.last().unwrap();
                }
            }
            //Columns aren't so straightforward with this representation of board
            //Assuming all of the boards are the same size
            for i in 0..board[0].len() {
                let mut tmp_column: Vec<i32> = Vec::new();
                for j in 0..board.len() {
                    tmp_column.push(board[j][i]);
                }
                if check_for_match(&draw, &tmp_column) && !winning.contains(board) {
                    winning.push(board.clone());
                    winning_no = *draw.last().unwrap();
                }
            }
        }
        if boards.len() == winning.len() {
            break;
        }
    }

    return sum_unmarked(&winning.pop().unwrap(), &draw) * winning_no;
}
fn part1(numbers: &Vec<i32>, boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut draw: Vec<i32> = Vec::new();
    let mut drawer = numbers.iter();
    while draw.len() != numbers.len() {
        draw.push(*drawer.next().unwrap());
        for board in boards {
            for row in board {
                if check_for_match(&draw, &row) {
                    return sum_unmarked(board, &draw) * draw.pop().unwrap();
                }
            }
            //Columns aren't so straightforward with this representation of board
            //Assuming all of the boards are the same size
            for i in 0..board[0].len() {
                let mut tmp_column: Vec<i32> = Vec::new();
                for j in 0..board.len() {
                    tmp_column.push(board[j][i]);
                }
                if check_for_match(&draw, &tmp_column) {
                    return sum_unmarked(board, &draw) * draw.pop().unwrap();
                }
            }
        }
    }
    0
}

fn sum_unmarked(board: &Vec<Vec<i32>>, marked: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for row in board {
        for value in row {
            if !marked.contains(value) {
                sum += value;
            }
        }
    }
    sum
}

fn check_for_match(pattern: &Vec<i32>, object: &Vec<i32>) -> bool {
    for number in object {
        if pattern.contains(number) {
            continue;
        }
        return false;
    }

    true
}
