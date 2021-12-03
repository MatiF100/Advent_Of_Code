use std::collections::HashMap;

#[derive(Debug)]
struct Data<'a> {
    ranges: HashMap<&'a str, [(usize, usize); 2]>,
    ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl<'a> Data<'a> {
    fn from_str(input: &str) -> Data {
        let input = input.split("\r\n\r\n").collect::<Vec<&str>>();

        let ranges = input[0]
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .map(|x| {
                        (
                            x.0,
                            x.1.split_once(" or ")
                                .expect("Error while reading valid value ranges"),
                        )
                    })
                    .map(|(name, ranges)| {
                        (
                            name,
                            [
                                Data::range_from_str(ranges.0),
                                Data::range_from_str(ranges.1),
                            ],
                        )
                    })
            })
            .map(|opt| opt.unwrap())
            .collect::<HashMap<&str, [(usize, usize); 2]>>();

        let ticket = input[1]
            .lines()
            .skip(1)
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .next()
            .unwrap();
        let nearby = input[2]
            .lines()
            .skip(1)
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<usize>>>();
        Data {
            ranges: ranges,
            ticket: ticket,
            nearby_tickets: nearby,
        }
    }
    fn range_from_str(range: &str) -> (usize, usize) {
        range
            .split_once("-")
            .map(|c| (c.0.parse::<usize>().unwrap(), c.1.parse::<usize>().unwrap()))
            .unwrap()
    }

    fn validate_tickets(&self) -> usize {
        let mut sum = 0;
        for ticket in &self.nearby_tickets {
            for field in ticket {
                if !self.validate_field(*field) {
                    sum += field;
                }
            }
        }
        sum
    }

    fn remove_invalid_tickets(&mut self) {
        self.nearby_tickets = self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.validate_ticket(ticket))
            .map(|r| r.iter().map(|x| *x).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
    }

    fn validate_ticket(&self, ticket: &Vec<usize>) -> bool {
        for field in ticket {
            if !self.validate_field(*field) {
                return false;
            }
        }
        true
    }

    fn validate_field(&self, field: usize) -> bool {
        for &ranges in self.ranges.values() {
            if field >= ranges[0].0 && field <= ranges[0].1
                || field >= ranges[1].0 && field <= ranges[1].1
            {
                return true;
            }
        }
        false
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut data = Data::from_str(&input);

    println!("Part 1: {}", data.validate_tickets());
    data.remove_invalid_tickets();
    dbg!(&data);
}
