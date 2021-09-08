#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Seat {
    EMPTY,
    OCCUPIED,
}

#[derive(Debug, Clone)]
struct Layout {
    seats: Vec<Vec<Option<Seat>>>,
}

impl Layout {
    fn from_str(data: &str) -> Self {
        Self {
            seats: data
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'L' => Some(Seat::EMPTY),
                            '#' => Some(Seat::OCCUPIED),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    fn reseat_until_stabilized(&mut self) {
        let mut prev = 0;
        self.perform_reseat();
        while self.count_occupied() != prev {
            prev = self.count_occupied();
            self.perform_reseat();
        }
    }

    fn reseat_until_stabilized_2(&mut self) {
        let mut prev = 0;
        self.perform_reseat_2();
        while self.count_occupied() != prev {
            prev = self.count_occupied();
            self.perform_reseat_2();
        }
    }
    fn perform_reseat(&mut self) {
        let initial_state = self.clone();
        for (row_idx, row) in initial_state.seats.iter().enumerate() {
            for (seat_idx, seat) in row.iter().enumerate() {
                self.seats[row_idx][seat_idx] = match seat {
                    Some(_) => initial_state.get_next_status(row_idx, seat_idx),
                    None => None,
                }
            }
        }
    }

    fn perform_reseat_2(&mut self) {
        let initial_state = self.clone();
        for (row_idx, row) in initial_state.seats.iter().enumerate() {
            for (seat_idx, seat) in row.iter().enumerate() {
                self.seats[row_idx][seat_idx] = match seat {
                    Some(_) => initial_state.get_next_status_2(row_idx, seat_idx),
                    None => None,
                }
            }
        }
    }


    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .map(|vec| {
                vec.iter()
                    .filter(|&seat| *seat == Some(Seat::OCCUPIED))
                    .count()
            })
            .sum()
    }

    fn get_next_status(&self, row_idx: usize, column_idx: usize) -> Option<Seat> {
        let mut adj_counter = 0;
        let lower_row_bound = if row_idx == 0 { 0 } else { row_idx - 1 };
        let lower_col_bound = if column_idx == 0 { 0 } else { column_idx - 1 };
        for i in lower_row_bound..=row_idx + 1 {
            for j in lower_col_bound..=column_idx + 1 {
                if i != row_idx || j != column_idx {
                    match self.seats.get(i) {
                        Some(row) => match row.get(j) {
                            Some(seat) => match seat {
                                Some(Seat::OCCUPIED) => adj_counter += 1,
                                _ => continue,
                            },
                            None => continue,
                        },
                        None => continue,
                    }
                }
            }
        }
        match (self.seats[row_idx][column_idx], adj_counter) {
            (Some(Seat::EMPTY), 0) => Some(Seat::OCCUPIED),
            (Some(Seat::OCCUPIED), 4..) => Some(Seat::EMPTY),
            _ => self.seats[row_idx][column_idx],
        }
    }

    fn get_next_status_2(&self, row_idx: usize, column_idx: usize) -> Option<Seat> {
        let mut adj_counter = 0;
        let (mut i, mut j) = (0, 0);

        //Top left diagonal
        loop {
            i+=1;
            j+=1;
            let (ii, jj) = match (row_idx.checked_sub(i), column_idx.checked_sub(j)) {
                (Some(a), Some(b)) => (a, b),
                _ => break,
            };
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }
        
        i=0;
        j=0;
        //Top vertical
        loop {
            i+=1; j+=1;
            let ii = match row_idx.checked_sub(i) {
                Some(a) => a,
                _ => break,
            };
            let jj = column_idx;
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }
        
        i=0;
        j=0;
        //Top right diagonal
        loop {
            i+=1; j+=1;
            let (ii, jj) = match (row_idx.checked_sub(i), column_idx.checked_add(j)) {
                (Some(a), Some(b)) => (a, b),
                _ => break,
            };
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }
        
        i=0;
        j=0;
        //Left Horizontal
        loop {
            i+=1;j+=1;
            let jj = match  column_idx.checked_sub(j) {
                Some(a) => a,
                _ => break,
            };
            let ii = row_idx;
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }
        
        //Center ommited - this is where we are reffering to
        
        i=0;
        j=0;
        //Right horizontal
        loop {
            i+=1; j+=1;
            let jj = match column_idx.checked_add(j) {
                Some(a) => a,
                _ => break,
            };
            let ii = row_idx;
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }

        i=0;
        j=0;
        //Bottom left diagonal
        loop {
            i+=1;
            j+=1;
            let (ii, jj) = match (row_idx.checked_add(i), column_idx.checked_sub(j)) {
                (Some(a), Some(b)) => (a, b),
                _ => break,
            };
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }
        
        i=0;
        j=0;
        //Bottom vertical
        loop {
            i+=1; j+=1;
            let ii = match row_idx.checked_add(i) {
                Some(a) => a,
                _ => break,
            };
            let jj = column_idx;
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }
        
        i=0;
        j=0;
        //Bottom right diagonal
        loop {
            i+=1; j+=1;
            let (ii, jj) = match (row_idx.checked_add(i), column_idx.checked_add(j)) {
                (Some(a), Some(b)) => (a, b),
                _ => break,
            };
            match self.seats.get(ii) {
                Some(row) => match row.get(jj) {
                    Some(seat) => match seat {
                        Some(Seat::OCCUPIED) => {
                            adj_counter += 1;
                            break;
                        }
                        Some(Seat::EMPTY) => break,
                        None => continue,
                    },
                    None => break,
                },
                None => break,
            };
        }

        match (self.seats[row_idx][column_idx], adj_counter) {
            (Some(Seat::EMPTY), 0) => Some(Seat::OCCUPIED),
            (Some(Seat::OCCUPIED), 5..) => Some(Seat::EMPTY),
            _ => self.seats[row_idx][column_idx],
        }
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut layout = Layout::from_str(&input);
    let mut layout_2 = layout.clone();
    layout.reseat_until_stabilized();
    println!("Part 1: {}", layout.count_occupied());
    layout_2.reseat_until_stabilized_2();
    println!("Part 2: {}", layout_2.count_occupied());
    //dbg!(&layout);
}
