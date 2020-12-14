use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
enum Command {
    Mask((u64, u64)),
    Write((usize, u64)),
}

fn parse_command(s: &str) -> Result<Command, ParseError> {
    lazy_static!{
        static ref RE_mask: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
        static ref RE_write: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let is_mask = RE_mask.is_match(s);

    if is_mask {
        let cap = RE_mask.captures(s).ok_or(ParseError::new(&format!("Could not parse '{}' as mask", s)))?;

        let mask: String = cap[1].chars()
            .map(|v| if v == 'X' {
                '1'
            } else {
                '0'
            })
            .collect();

        let overwrite: String = cap[1].chars()
            .map(|v| if v == '1' {
                '1'
            } else {
                '0'
            })
            .collect();

        let mask = u64::from_str_radix(&mask, 2)?;
        let overwrite = u64::from_str_radix(&overwrite, 2)?;
        Ok(Command::Mask((mask, overwrite)))
    } else {
        let cap = RE_write.captures(s).ok_or(ParseError::new(&format!("Could not parse '{}' as write", s)))?;
        let address = cap[1].parse::<usize>()?;
        let value = cap[2].parse::<u64>()?;
        Ok(Command::Write((address, value)))
    }
}

fn parse_input() -> Result<Vec<Command>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| parse_command(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let commands = parse_input()?;

    let buffer_max = commands.iter()
        .map(|c| match c {
            Command::Write((a, _)) => *a,
            Command::Mask(_) => 0,
        })
        .max()
        .ok_or(ParseError::new("Could not determine memory size."))?;

    let mut memory = vec![0; buffer_max + 1];
    let mut mask = (0, 0);

    for c in &commands {
        match c {
            Command::Mask(m) => {
                mask = *m;
            },
            Command::Write((a, v)) => {
                memory[*a] = (v & mask.0) | mask.1;
            }
        }
    }

    let result: u64 = memory.iter()
        .filter(|&v| *v != 0)
        .sum();

    println!("14/1: memory init result is: {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input();

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