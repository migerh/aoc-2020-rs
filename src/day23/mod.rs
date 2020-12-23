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
    let three = vec![
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap()
    ];
    // for _ in 0..3 {
    //     three.push(cups.pop_front().unwrap());
    // }
    cups.push_front(first);
    three
}

fn get_destination(max: u64, destination: u64) -> u64 {
    if destination == 1 {
        max as u64
    } else {
        destination - 1
    }
}

fn insert_three(cups: &mut VecDeque<u64>, max: u64, three: &Vec<u64>) {
    let current = cups[0];

    let mut destination = get_destination(max, current);
    while three.contains(&destination) {
        destination = get_destination(max, destination);
    }
    // println!("destination: {}", destination);

    let destination_position = cups.iter().position(|v| *v == destination).unwrap();
    cups.insert(destination_position + 1, three[2]);
    cups.insert(destination_position + 1, three[1]);
    cups.insert(destination_position + 1, three[0]);
    // for t in three.iter().rev() {
    //     cups.insert(destination_position + 1, *t);
    // }
}

fn rotate_to_new_current(cups: &mut VecDeque<u64>, current: u64) {
    while cups[0] != current {
        cups.rotate_left(1);
    }
    let c = cups.pop_front().unwrap();
    cups.push_back(c);
}

fn turn(cups: &mut VecDeque<u64>, max: u64) {
    let current = cups[0];
    let three = pick_three(cups);
    // println!("pick up: {:?}", three);
    insert_three(cups, max, &three);
    rotate_to_new_current(cups, current);
}

fn rotate_to_1(cups: &mut VecDeque<u64>) {
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
}

fn checksum(cups: &mut VecDeque<u64>) -> String {
    rotate_to_1(cups);
    join(cups.iter().skip(1), "")
}

pub fn problem1() -> Result<(), ParseError> {
    let mut cups = get_input();

    for t in 0..100 {
        println!("--- move {} ---", t + 1);
        println!("cups: {:?}", cups);
        turn(&mut cups, 9);
        println!("");
    }

    println!("result: {}", checksum(&mut cups));

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let mut cups = (1..=1_000_000).collect::<VecDeque<u64>>();
    let first_10 = get_input();

    for (i, v) in first_10.into_iter().enumerate() {
        cups[i] = v;
    }

    for t in 0..10_000_000 {
    // for t in 0..10 {
        if t % 10_000 == 0 {
            println!("move {}", t);
        }
        // println!("--- move {} ---", t + 1);
        // println!("cups: {:?}", cups);
        turn(&mut cups, 1_000_000);
        // println!("");
    }
    rotate_to_1(&mut cups);

    let star1 = cups[1];
    let star2 = cups[2];

    println!("result: {}", star1 * star2);

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