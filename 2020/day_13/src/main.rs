fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<&str>>();
    let timestamp = input[0].parse::<i32>().unwrap();
    let buses = input[1].split(',').collect::<Vec<&str>>();
    let valid_buses = buses
        .iter()
        .filter(|&bus| *bus != "x")
        .map(|bus| bus.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let p1 = find_nearest_bus(timestamp, &valid_buses);
    println!("Part 1: {}", p1.0 * p1.1);
    println!("Part 2: {}", search_for_sequence(&buses));
}

fn find_nearest_bus(timestamp: i32, buses: &Vec<i32>) -> (i32, i32) {
    buses
        .iter()
        .map(|bus| (bus - timestamp % bus, *bus))
        .min_by(|(x, _), (y, _)| x.cmp(y))
        .unwrap()
}

fn search_for_sequence(buses: &Vec<&str>) -> usize {
    let valid_buses = buses
        .iter()
        .enumerate()
        .filter(|&(_, bus)| *bus != "x")
        .map(|(idx, bus)| (idx, bus.parse::<usize>().unwrap()))
        .map(|(idx, bus)| {
            (
                match bus.checked_sub(idx) {
                    Some(dif) => dif,
                    
                    //I have gotten to this using paint, screen capture and a ton of weird logic... Don't ask me how i got this None variant to work
                    None => bus - (idx % bus) ,
                }%bus,
                bus,
            )
        })
        .collect::<Vec<(usize, usize)>>();
    //let remainders = valid_buses.iter().map(|(idx, bus)| bus - idx).collect::<Vec<usize>>();

    let mut prev = valid_buses[0];
    for bus in valid_buses.iter().skip(1) {
        prev = crt(prev, *bus);
    }

    prev.0
}

//Chinese Remainder Theorem, returns lowest solution and the LCM increment
fn crt(x: (usize, usize), y: (usize, usize)) -> (usize, usize) {
    let mut ret = x.0;
    let dif = lcm(x.1, y.1);
    while ret % y.1 != y.0 {
        ret += x.1;
    }
    (ret, dif)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
