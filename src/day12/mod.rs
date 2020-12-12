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
    waypoint: Position,
    direction: char,
}

impl Navigation {
    fn waypoint(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let delta = Position::delta(update.direction, update.distance)?;

        self.waypoint = self.waypoint.travel(&delta);
        Ok(())
    }

    fn movement(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let delta = Position::new(update.distance * self.waypoint.x, update.distance * self.waypoint.y);
        let new_position = self.position.travel(&delta);

        self.position = new_position;
        Ok(())
    }

    fn rotation(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let x = self.waypoint.x;
        let y = self. waypoint.y;

        let waypoint = match (update.direction, update.distance) {
            ('L', 90) => (-y, x),
            ('L', 180) => (-x, -y),
            ('L', 270) => (y, -x),

            ('R', 90) => (y, -x),
            ('R', 180) => (-x, -y),
            ('R', 270) => (-y, x),

            _ => Err(ParseError::new(&format!("Invalid direction update: '{:?}'", update)))?,
        };

        self.waypoint = Position::new(waypoint.0, waypoint.1);

        Ok(())
    }

    pub fn init() -> Self {
        Navigation {
            direction: 'E',
            position: Position::new(0, 0),
            waypoint: Position::new(10, 1)
        }
    }

    pub fn travel(&mut self, update: &NavigationUpdate) -> Result<(), ParseError> {
        let waypoint = vec!['N', 'S', 'E', 'W'];
        let movement = vec!['F'];
        let rotation = vec!['L', 'R'];

        if waypoint.contains(&update.direction) {
            self.waypoint(update)
        } else if movement.contains(&update.direction) {
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

    println!("12/2: manhattan distance: {}", ship.position.manhattan());

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    Ok(())
}
