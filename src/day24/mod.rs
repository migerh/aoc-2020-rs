use std::collections::HashMap;
use std::ops::Add;
use super::utils::ParseError;

#[derive(Debug)]
enum Direction {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

fn parse_line(s: &str) -> Vec<Direction> {
    let mut i = s.chars().peekable();
    let mut result = vec![];

    while let Some(c) = i.next() {
        let p = i.peek();

        let d = match (c, p) {
            ('s', Some('e')) => {
                i.next();
                Direction::SouthEast
            },
            ('s', Some('w')) => {
                i.next();
                Direction::SouthWest
            },
            ('w', _) => {
                Direction::West
            },
            ('e', _) => {
                Direction::East
            },
            ('n', Some('e')) => {
                i.next();
                Direction::NorthEast
            },
            ('n', Some('w')) => {
                i.next();
                Direction::NorthWest
            },
            (v1, v2) => panic!(format!("Encountered unexpected pair of input chars: '{}' and '{:?}'", v1, v2))
        };

        result.push(d);
    }

    result
}

fn parse_input() -> Vec<Vec<Direction>> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| parse_line(v))
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

impl Coords {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn zeroes() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn from_direction(d: &Direction) -> Self {
        let (x, y, z) = match d {
            Direction::NorthEast => (1, -1, 0),
            Direction::East => (1, 0, -1),
            Direction::SouthEast => (0, 1, -1),
            Direction::SouthWest => (-1, 1, 0),
            Direction::West => (-1, 0, 1),
            Direction::NorthWest => (0, -1, 1),
        };

        Self { x, y, z }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn get_tile(instructions: &Vec<Direction>) -> Coords {
    let tile = instructions.iter()
        .map(|d| Coords::from_direction(d))
        .fold(Coords::zeroes(), |a, c| a + c);

    tile
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input();

    let mut floor: HashMap<Coords, bool> = HashMap::new();
    for tile in input.iter() {
        let coords = get_tile(tile);
        floor.entry(coords)
            .and_modify(|v| *v = !*v)
            .or_insert(true);
    }

    let result = floor.iter()
        .filter(|(_, v)| **v)
        .count();

    println!("24/1: # of flipped tiles: {}", result);

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