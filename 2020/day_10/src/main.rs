use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    input.push(input.iter().max().unwrap() + 3);
    input.push(0);
    input.sort();
    input.reverse();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(list: &Vec<usize>) -> i32 {
    let mut diff = [0; 3];
    let mut prev = 0;
    for i in 1..list.len() {
        diff[list[prev] - list[i] - 1] += 1;
        prev = i;
    }

    diff[0] * diff[2]
}

fn part2(list: &Vec<usize>) -> usize {
    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    let top = list.iter().max().unwrap();
    for adapter in list {
        connections.insert(
            *adapter,
            list.iter()
                .filter(|jolt| match adapter.checked_sub(**jolt) {
                    Some(x) => {
                        if x <= 3 && x != 0{
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                })
                .map(|&x| x)
                .collect(),
        );
    }
    get_adapter_connections_rec(&mut sizes, &connections, *top);
    *sizes.get(top).unwrap()
}

fn get_adapter_connections_rec(known: &mut HashMap<usize, usize>, connections: &HashMap<usize, Vec<usize>>, entry_adapter: usize) -> usize{
        if let Some(x) = known.get(&entry_adapter){
            return *x;
        }
        else if connections.get(&entry_adapter).unwrap().len() == 0{
            return 1;
        }
        else{
            let mut sum = 0;
            for compatibles in connections.get(&entry_adapter){
                for compatible in compatibles{
                    let x = get_adapter_connections_rec(known, connections, *compatible);
                    sum += x;
                }
            }
            known.insert(entry_adapter, sum);
            return sum;
        }
        
}
