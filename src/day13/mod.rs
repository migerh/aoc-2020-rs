use std::iter;
use super::utils::ParseError;

#[derive(Debug)]
struct Terminal {
    timestamp: usize,
    busses: Vec<Option<usize>>,
}

fn parse_input() -> Result<Terminal, ParseError> {
    let input = include_str!("./data/input.txt");
    let lines = input
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    let timestamp = lines[0].parse::<usize>()?;
    let busses = lines[1]
        .split(',')
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    Ok(Terminal { timestamp, busses })
}

pub fn problem1() -> Result<(), ParseError> {
    let terminal = parse_input()?;

    let mut next_arrivals = terminal.busses.iter()
        .filter(|b| b.is_some())
        .map(|b| b.unwrap())
        .map(|b| (b, (terminal.timestamp - (terminal.timestamp % b)) + b))
        .collect::<Vec<_>>();

    next_arrivals.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let next = next_arrivals.iter().next().ok_or(ParseError::new("No bus found"))?;

    let arrives_in = next.1 - terminal.timestamp;
    println!("Next arriving bus is {} in {} minutes.", next.0, arrives_in);
    println!("13/1: checksum is {}", next.0 * arrives_in);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    // hardcoded the problem because Rust iterators are statically typed
    // and I couldn't find a way to concatenate filters dynamically because
    // of that.
    // This takes a lot of time!
    let initial = iter::repeat(1)
        .enumerate()
        .map(|(index, _)| index as usize)
        .skip(166389351081 * 601 - 60 as usize)
        .step_by(601)

        // hard wire the problem
        // .filter(|v| (v + 60) % 601 == 0)
        .filter(|v| (v + 29) % 577 == 0 &&
            (v + 19) % 41 == 0 &&
            (v + 97) % 37 == 0 &&
            (v + 0) % 29 == 0 &&
            (v + 52) % 23 == 0 &&
            (v + 48) % 19 == 0)
        // get some kind of "progress" notification
        .map(|v| {
            println!("{}", v);
            v
        })
        .filter(|v| (v + 43) % 17 == 0)
        .filter(|v| (v + 42) % 13 == 0)
        .take(1)
        .collect::<Vec<_>>();

    println!("result: {:?}", initial);

    Ok(())
}
