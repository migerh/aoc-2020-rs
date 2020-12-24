use super::utils::ParseError;

#[derive(Debug)]
enum Directions {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

fn parse_line(s: &str) -> Vec<Directions> {
    let mut i = s.chars().peekable();
    let mut result = vec![];

    while let Some(c) = i.next() {
        let p = i.peek();

        let d = match (c, p) {
            ('s', Some('e')) => {
                i.next();
                Directions::SouthEast
            },
            ('s', Some('w')) => {
                i.next();
                Directions::SouthWest
            },
            ('w', _) => {
                Directions::West
            },
            ('e', _) => {
                Directions::East
            },
            ('n', Some('e')) => {
                i.next();
                Directions::NorthEast
            },
            ('n', Some('w')) => {
                i.next();
                Directions::NorthWest
            },
            (v1, v2) => panic!(format!("Encountered unexpected pair of input chars: '{}' and '{:?}'", v1, v2))
        };

        result.push(d);
    }

    result
}

fn parse_input() -> Vec<Vec<Directions>> {
    let input = include_str!("./data/example.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| parse_line(v))
        .collect::<Vec<_>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input();

    println!("{:?}", input[2]);

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