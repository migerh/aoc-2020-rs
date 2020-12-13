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

fn chinese_remainder(start: usize, remainder: &Vec<(usize, usize)>) -> usize {
    let lb = start;
    let mut t = lb + (lb % remainder[0].1) + remainder[0].0;
    let mut inc = remainder[0].1;

    // search the solution with the chinese remainder theorem
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
    for bus in remainder.iter().skip(1) {
        loop {
            if t % bus.1 == bus.0 {
                break;
            }

            t += inc;
        }

        inc *= bus.1;
    }

    t
}

pub fn problem2() -> Result<(), ParseError> {
    let terminal = parse_input()?;

    let mut busses = terminal.busses.into_iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| ((b.unwrap() - i % b.unwrap()) % b.unwrap(), b.unwrap()))
        .collect::<Vec<_>>();

    busses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let result = chinese_remainder(0, &busses);
    println!("13/2: result {}", result);

    Ok(())
}