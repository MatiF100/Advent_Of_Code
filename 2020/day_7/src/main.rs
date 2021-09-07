use std::collections::HashMap;

type Rules<'a> = HashMap<Bag<'a>, HashMap<Bag<'a>, i32>>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Bag<'a> {
    attribute: &'a str,
    color: &'a str,
}

impl<'a> Bag<'a> {
    fn new(color: &'a str, attribute: &'a str) -> Self {
        Self {
            attribute: attribute,
            color: color,
        }
    }
    fn from_string(recipe: &'a str) -> Self {
        let parameters = Self::get_bag_parameters(recipe);
        Self {
            attribute: parameters.0,
            color: parameters.1,
        }
    }

    fn get_bag_parameters(bag: &'a str) -> (&'a str, &'a str) {
        let bag = bag.split_ascii_whitespace().collect::<Vec<&str>>();
        (bag[0], bag[1])
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .lines()
        .map(|line| match line.split_once(" contain ") {
            Some((product, ingredients)) => {
                let prod = deserialize_bag_info(product)
                    .expect("Error while getting outer bag info! Aborting...")
                    .1;
                let in_list = ingredients
                    .split(", ")
                    .map(|s| deserialize_bag_info(s))
                    .filter(|opt| opt.is_some())
                    .map(|some| some.unwrap())
                    .map(|(count, bag)| (bag, count))
                    .collect::<HashMap<Bag, i32>>();
                //println!("{:?}", in_list);
                //println!("Bag: {:?}, Contains: {:?}", prod, in_list);
                (prod, in_list)
            }
            None => panic!("Problem while parsing recipe! Aborting now..."),
        })
        .collect::<Rules>();
    //println!("{:?}", input);
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn part1(input: Rules) -> usize {
    let mut bag_list: HashMap<Bag, i32> = HashMap::new();
    for bag in input.keys() {
        search_bag_rec(&mut bag_list, &input, bag, &Bag::new("gold", "shiny"));
    }
    //println!("{:?}",bag_list);
    //dbg!(&bag_list);
    bag_list.iter().filter(|&(_, status)| *status == 1).count()
}
fn part2(input: Rules) -> i32 {
    let mut bag_list: HashMap<Bag, i32> = HashMap::new();
    fill_bag_rec(&mut bag_list, &input, &Bag::new("gold", "shiny")) - 1
}

fn fill_bag_rec<'a>(list: &mut HashMap<Bag<'a>, i32>, rules: &Rules<'a>, bag: &Bag<'a>) -> i32 {
    let inner = rules.get(bag).unwrap();
    if let Some(target) = list.get(bag) {
        return *target + 1;
    } else if inner.is_empty() {
        list.insert(bag.clone(), 0);
        return 1;
    } else {
        let mut sum = 0;
        for (element, count) in inner {
            let value = fill_bag_rec(list, rules, element);
            sum += count * value;
        }
        list.insert(bag.clone(), sum);
        return sum + 1;
    }
}

fn search_bag_rec<'a>(
    list: &mut HashMap<Bag<'a>, i32>,
    rules: &Rules<'a>,
    bag: &Bag<'a>,
    search: &Bag,
) -> bool {
    let inner = rules.get(bag).unwrap();
    if let Some(_) = inner.get(search) {
        list.insert(bag.clone(), 1);
        return true;
    }
    if let Some(target) = list.get(bag) {
        if *target > 0 {
            return true;
        } else {
            return false;
        }
    } else if inner.is_empty() || bag == search {
        list.insert(bag.clone(), -1);
        return false;
    } else {
        for (element, _) in inner {
            match search_bag_rec(list, rules, element, search) {
                true => {
                    list.insert(bag.clone(), 1);
                    return true;
                }
                false => (),
            }
        }
    }

    false
}

fn deserialize_bag_info(info: &str) -> Option<(i32, Bag)> {
    let words = info.split_ascii_whitespace().count();
    //println!("{:?}", info);
    match words {
        3 => match info {
            "no other bags." => None,
            _ => Some((1, Bag::from_string(info))),
        },
        4 => Some((
            info.split_once(" ").unwrap().0.parse::<i32>().unwrap(),
            //WARNING! Assuming that every bag contains at most 9 bags of other given color
            Bag::from_string(&info[2..]),
        )),
        _ => None,
    }
}
