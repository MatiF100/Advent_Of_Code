use num::bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num::cast::ToPrimitive;
fn main() {
    let input = if let Ok(s) = std::fs::read_to_string("input.txt") {
        s
    } else {
        "3,4,3,1,2".to_owned()
    };
    let input = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 2: {}", calc(&input, 80));
    println!("Part 2: {}", calc(&input, 256));
    println!("The following can take A LOT of time, since value passed is 10 million. If you wish, you can adjust it as necessary");
    let input = input.iter().map(|x| x.to_bigint().unwrap()).collect::<Vec<_>>();
    println!("bench: {}", calc_bi(&input, 10.to_biguint().unwrap().pow(7)));
}

fn calc(init: &Vec<i64>, count: usize) -> i128{
    //7 days is a cycle
    //1 day is a phase
    let mut states:[i128; 9] = [0; 9];
    let init:Vec<i128> = init.iter().map(|i| *i as i128).collect();

    init.iter().for_each(|x| states[*x as usize] += 1);
    for _ in 0..count{
        //get new "phase" of current "cycle" children
        let new = states[0];
        for state in 0..8{
            states[state] = states[state+1];
        }
        states[8] = new;
        states[6] += new;
        
    }
    states.iter().sum::<i128>()
}

fn calc_bi(init: &Vec<BigInt>, count: BigUint) -> BigInt {
    //7 days is a cycle
    //1 day is a phase
    let mut states: [BigInt; 9] = [
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
        0i32.to_bigint().unwrap(),
    ];

    init.iter()
        .for_each(|x| states[(*x).to_usize().unwrap()] += 1);
    let mut cnt = 0.to_biguint().unwrap();
    while cnt < count{
    //for _ in 0..count {
        states = [
            states[1].clone(),
            states[2].clone(),
            states[3].clone(),
            states[4].clone(),
            states[5].clone(),
            states[6].clone(),
            states[7].clone() + states[0].clone(),
            states[8].clone(),
            states[0].clone(),
        ];
        cnt = cnt + 1.to_biguint().unwrap();
    }
    states.iter().sum()
}