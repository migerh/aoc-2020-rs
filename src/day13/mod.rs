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
    let terminal = parse_input()?;

    let mut busses = terminal.busses.into_iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| (i, b.unwrap()))
        .collect::<Vec<_>>();

    busses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let lb = 100000000000000_usize;
    // let lb = 0_usize;
    let mut t = lb + (lb % busses[0].1) - busses[0].0;

    'main: loop {
        let mut done = true;
        for bus in busses.iter().skip(1) {
            done &= (t + bus.0) % bus.1 == 0;
            if !done {
                t += busses[0].1;
                continue 'main;
            }
        }

        if done {
            break;
        }

        t += busses[0].1;
    }

    println!("result: {}", t);

    Ok(())
}
