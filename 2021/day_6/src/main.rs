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
    println!("The following can crash for your input data as i128 is a bit limited. Please adjust the limit if it does so");
    println!("bench: {}", calc(&input, 943));
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
