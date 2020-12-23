use std::collections::HashMap;
use std::collections::VecDeque;
use itertools::join;
use super::utils::ParseError;

fn get_input() -> Vec<usize> {
    vec![4, 6, 3, 5, 2, 8, 1, 7, 9]
}

fn get_example() -> Vec<usize> {
    vec![3, 8, 9, 1, 2, 5, 4, 6, 7]
}

fn input_to_map(input: &Vec<usize>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for i in 0..input.len()-1 {
        map.entry(input[i]).or_insert(input[i+1]);
    }
    map.entry(*input.last().unwrap()).or_insert(*input.first().unwrap());
    map
}

struct State {
    current: usize,
    max: usize,
    map: HashMap<usize, usize>,
}

impl State {
    fn print(&self) {
        let mut result = vec![];

        let mut current = self.current;
        while result.len() != self.map.len() {
            result.push(current);
            current = self.map[&current];
        }
        println!("{:?}", result);
    }
}

fn pick_three(state: &mut State) -> usize {
    state.map[&state.current]
}

fn get_destination(max: usize, destination: usize) -> usize {
    if destination == 1 {
        max
    } else {
        destination - 1
    }
}

fn turn(state: &mut State) {
    let three = pick_three(state);

    let first = three;
    let second = state.map[&first];
    let third = state.map[&second];

    println!("pick up: {}, {}, {}", first, second, third);

    let mut destination = get_destination(state.max, state.current);
    while first == destination || second == destination || third == destination {
        destination = get_destination(state.max, destination);
    }
    println!("destination: {}", destination);

    let after_destination = state.map[&destination];
    let after_third = state.map[&third];
    state.map.entry(destination).and_modify(|v| *v = first);
    state.map.entry(third).and_modify(|v| *v = after_destination);
    state.map.entry(state.current).and_modify(|v| *v = after_third);

    state.current = state.map[&state.current];
}

fn rotate_to_1(cups: &mut VecDeque<u64>) {
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
}

fn checksum(state: &State) -> String {
    let mut result = vec![];
    let mut previous = 1;

    for i in 1..9 {
        previous = state.map[&previous];
        result.push(previous);
    }
    join(result, "")
}

pub fn problem1() -> Result<(), ParseError> {
    let input = get_input();
    let cups = input_to_map(&input);
    let mut state = State { current: input[0], max: 9, map: cups };

    for t in 0..100 {
        println!("--- move {} ---", t + 1);
        print!("cups: ");
        state.print();
        turn(&mut state);
        println!("");
    }

    println!("result: {}", checksum(&state));

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    // let mut cups = (1..=1_000_000).collect::<VecDeque<u64>>();
    // let first_10 = get_input();

    // for (i, v) in first_10.into_iter().enumerate() {
    //     cups[i] = v;
    // }

    // for t in 0..10_000_000 {
    // // for t in 0..10 {
    //     if t % 10_000 == 0 {
    //         println!("move {}", t);
    //     }
    //     // println!("--- move {} ---", t + 1);
    //     // println!("cups: {:?}", cups);
    //     turn(&mut cups);
    //     // println!("");
    // }
    // rotate_to_1(&mut cups);

    // let star1 = cups[1];
    // let star2 = cups[2];

    // println!("result: {}", star1 * star2);

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