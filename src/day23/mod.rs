use std::collections::VecDeque;
use itertools::join;
use super::utils::ParseError;

fn get_input() -> VecDeque<u64> {
    vec![4, 6, 3, 5, 2, 8, 1, 7, 9].into_iter().collect()
}

fn get_example() -> VecDeque<u64> {
    vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect()
}

fn pick_three(cups: &mut VecDeque<u64>) -> Vec<u64> {
    let first = cups.pop_front().unwrap();
    let mut three = vec![];
    for _ in 0..3 {
        three.push(cups.pop_front().unwrap());
    }
    cups.push_front(first);
    three
}

fn rotate_to_destination(cups: &mut VecDeque<u64>) {
    let mut remaining = cups.iter().cloned().collect::<Vec<_>>();
    remaining.sort_by(|a, b| b.cmp(a));
    let first = cups[0];

    let first_position = remaining.iter().position(|v| *v == first).unwrap();
    let destination_position = (first_position + 1) % remaining.len();
    let destination = remaining[destination_position];

    while cups[0] != destination {
        cups.rotate_left(1);
    }
}

fn push_three(cups: &mut VecDeque<u64>, three: &Vec<u64>) {
    let first = cups.pop_front().unwrap();
    for t in three.iter().rev() {
        cups.push_front(*t);
    }
    cups.push_front(first);
}

fn rotate_to_new_current(cups: &mut VecDeque<u64>, current: u64) {
    while cups[0] != current {
        cups.rotate_left(1);
    }
    let c = cups.pop_front().unwrap();
    cups.push_back(c);
}

fn turn(cups: &mut VecDeque<u64>) {
    let current = cups[0];
    let three = pick_three(cups);
    println!("pick up: {:?}", three);
    rotate_to_destination(cups);
    println!("destination: {}", cups[0]);
    push_three(cups, &three);
    rotate_to_new_current(cups, current);
}

fn checksum(cups: &mut VecDeque<u64>) -> String {
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
    join(cups.iter().skip(1), "")
}

pub fn problem1() -> Result<(), ParseError> {
    let mut cups = get_input();

    for t in 0..100 {
        println!("--- move {} ---", t + 1);
        println!("cups: {:?}", cups);
        turn(&mut cups);
        println!("");
    }

    println!("result: {}", checksum(&mut cups));

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let cups = get_input();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
    }

    #[test]
    pub fn example_2_1() {
    }
}