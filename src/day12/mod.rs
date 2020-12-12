use std::str::FromStr;
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
struct NavigationUpdate {
    direction: char,
    distance: i32,
}

impl FromStr for NavigationUpdate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(\w)(\d+)$").unwrap();
        }

        let cap = RE.captures(s).unwrap();
        let direction = cap[1].chars().next().ok_or(ParseError::new(&format!("Unable to parse input: '{}'", s)))?;
        let distance = cap[2].parse::<i32>()?;

        Ok(Self { direction, distance })
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn delta(direction: char, distance: i32) -> Result<Position, ParseError> {
        let (x, y) = match direction {
            'N' => (0, distance),
            'E' => (distance, 0),
            'S' => (0, -distance),
            'W' => (-distance, 0),
            e => Err(ParseError::new(&format!("Invalid direction: '{}'", e)))?
        };

        Ok(Position::new(x, y))
    }

    pub fn travel(&self, delta: &Position) -> Position {
        Position { x: self.x + delta.x, y: self.y + delta.y }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Navigation {
    position: Position,
    direction: char,
}

impl Navigation {
    fn movement(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let delta = if update.direction == 'F' {
            Position::delta(self.direction, update.distance)
        } else {
            Position::delta(update.direction, update.distance)
        }?;

        let new_position = self.position.travel(&delta);

        self.position = new_position;
        Ok(())
    }

    fn rotation(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let new_direction = match (self.direction, update.direction, update.distance) {
            ('N', 'L', 90) => 'W',
            ('N', 'L', 180) => 'S',
            ('N', 'L', 270) => 'E',
            ('E', 'L', 90) => 'N',
            ('E', 'L', 180) => 'W',
            ('E', 'L', 270) => 'S',
            ('S', 'L', 90) => 'E',
            ('S', 'L', 180) => 'N',
            ('S', 'L', 270) => 'W',
            ('W', 'L', 90) => 'S',
            ('W', 'L', 180) => 'E',
            ('W', 'L', 270) => 'N',

            ('N', 'R', 90) => 'E',
            ('N', 'R', 180) => 'S',
            ('N', 'R', 270) => 'W',
            ('E', 'R', 90) => 'S',
            ('E', 'R', 180) => 'W',
            ('E', 'R', 270) => 'N',
            ('S', 'R', 90) => 'W',
            ('S', 'R', 180) => 'N',
            ('S', 'R', 270) => 'E',
            ('W', 'R', 90) => 'N',
            ('W', 'R', 180) => 'E',
            ('W', 'R', 270) => 'S',

            _ => Err(ParseError::new(&format!("Invalid direction update: '{:?}'", update)))?,
        };

        self.direction = new_direction;
        Ok(())
    }

    pub fn init() -> Self {
        Navigation { direction: 'E', position: Position::new(0, 0) }
    }

    pub fn travel(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let movement = vec!['N', 'S', 'E', 'W', 'F'];
        let rotation = vec!['L', 'R'];

        if movement.contains(&update.direction) {
            self.movement(update)
        } else if rotation.contains(&update.direction) {
            self.rotation(update)
        } else {
            Err(ParseError::new(&format!("Invalid command: '{}'", update.direction)))
        }
    }
}

fn parse_input() -> Result<Vec<NavigationUpdate>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| NavigationUpdate::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input()?;

    let mut ship = Navigation::init();

    for update in input {
        ship.travel(&update)?;
    }

    println!("12/1: manhattan distance: {}", ship.position.manhattan());

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input()?;

    Ok(())
}
