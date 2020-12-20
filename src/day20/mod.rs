use std::str::FromStr;
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
struct Tile {
    id: u64,
    data: Vec<Vec<char>>,
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }

        let id_str = s.lines().take(1).next().ok_or(ParseError::new(&format!("Could not find tile id in {}", s)))?;

        let cap = RE.captures(id_str).ok_or(ParseError::new(&format!("Could not extract id from tile header: {}", id_str)))?;
        let id = cap[1].parse::<u64>()?;
        let data = s.lines().skip(1).map(|v| v.chars().collect()).collect();
        Ok(Self { id, data })
    }
}

impl Tile {
    fn print(&self) {
        println!("Tile {}:", self.id);
        for line in &self.data {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }

    fn hash_line(line: &Vec<char>) -> u64 {
        line.iter().enumerate()
            .fold(0, |acc, (i, c)| acc + if c == &'#' { 1u64 << i as u64 } else { 0 })
    }

    fn hashes(&self) -> TileHash {
        let top = Self::hash_line(&self.data[0]);
        let right = Self::hash_line(&self.data.iter().map(|v| v[9]).collect::<Vec<_>>());

        // By flipping the next two hashes we make the hashes of the tile
        // rotation invariant
        let bottom = TileHash::flip(Self::hash_line(&self.data[9]));
        let left = TileHash::flip(Self::hash_line(&self.data.iter().map(|v| v[0]).collect::<Vec<_>>()));

        TileHash { id: self.id, data: vec![top, right, bottom, left] }
    }
}

#[derive(Debug)]
struct TileHash {
    id: u64,
    data: Vec<u64>,
}

impl TileHash {
    fn flip(hash: u64) -> u64 {
        let mut flip = 0;
        for i in 0..10 {
            let bit = 1 << i;
            flip += if hash & bit != 0 { 1 } else { 0 } << (9 - i);
        }
        flip
    }

    fn number_of_neighbors(&self, all_hashes: &Vec<TileHash>) -> u64 {
        // we could account for flipping in here by inverting the hashes...
        let other_tiles = all_hashes.iter().filter(|t| t.id != self.id).collect::<Vec<_>>();

        let mut count = 0;
        for h in &self.data {
            count += other_tiles.iter()
                .filter(|oh| oh.data.contains(h))
                .count();
        }

        // do everything again but flip it
        for h in self.data.iter().map(|h| Self::flip(*h)) {
            count += other_tiles.iter()
                .filter(|oh| oh.data.contains(&h))
                .count();
        }

        count as u64
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for h in &self.data {
            print!("{}  ", h);
        }
        println!("");
    }
}

struct TileConnection {
    my_border: usize,
    next_tile: u64,
    next_border: usize,
    flipped: bool,
}

fn parse_input() -> Result<Vec<Tile>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .split("\n\n")
        .filter(|v| *v != "")
        .map(|v| Tile::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input()?;

    let hashes = input.iter()
        .map(|t| t.hashes())
        .collect::<Vec<_>>();

    let result: u64 = hashes.iter()
        .map(|h| (h.id, h.number_of_neighbors(&hashes)))
        .filter(|n| n.1 == 2)
        .map(|h| h.0)
        .product();

    println!("20/1: Product of the ids of all four corners of the map: {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input()?;

    let hashes = input.iter()
        .map(|t| t.hashes())
        .collect::<Vec<_>>();

    // for hash in &hashes {
    //     hash.print();
    // }

    let corners = hashes.iter()
        .map(|h| (h.id, h.number_of_neighbors(&hashes)))
        // .filter(|n| n.1 == 2)
        .collect::<Vec<_>>();

    println!("corners: {:?}", corners.iter().filter(|t| t.1 == 2).count());
    println!("borders: {:?}", corners.iter().filter(|t| t.1 == 3).count());
    println!("inside: {:?}", corners.iter().filter(|t| t.1 == 4).count());

    // let result: u64 = corners.iter()
    //     .map(|h| h.0)
    //     .product();
    // println!("20/2: # of irrelevant thingies: {}", result);

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