use std::collections::HashSet;
use std::collections::HashMap;
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
        let mut data = s.lines().skip(1).map(|v| v.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        for y in 1..(data.len()-1) {
            for x in 1..(data[y].len()-1) {
                data[y][x] = ' ';
            }
        }
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

    fn get_line(&self, line: usize, rotation: usize, y_flipped: bool, x_flipped: bool) -> Vec<char> {
        let line = if y_flipped { 9 - line } else { line };

        // mighty inefficient, but it's at least somewhat recognizable what is
        // happening in here.
        let rotated_line = if rotation == 0 {
            self.data[line].clone()
        } else if rotation == 1 {
            self.data.iter().map(|v| v[line]).rev().collect::<Vec<_>>()
        } else if rotation == 2 {
            self.data[9 - line].iter().rev().cloned().collect::<Vec<_>>()
        } else if rotation == 3 {
            self.data.iter().map(|v| v[9 - line]).collect::<Vec<_>>()
        } else {
            panic!(format!("Unknown rotation: {}", rotation));
        };

        let flipped_line = if x_flipped {
            rotated_line.iter().rev().cloned().collect::<Vec<_>>()
        } else {
            rotated_line
        };

        flipped_line
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
        self.find_neighbors(all_hashes).len() as u64
    }

    fn find_neighbors(&self, all_hashes: &Vec<TileHash>) -> Vec<TileConnection> {
        // we could account for flipping in here by inverting the hashes...
        let other_tiles = all_hashes.iter().filter(|t| t.id != self.id).collect::<Vec<_>>();

        let find = |my_border: usize, my_border_hash: &u64, flipped: bool| {
            other_tiles.iter()
                .filter_map(|neighbor| {
                    let matching_borders = neighbor.data.iter().enumerate()
                        .filter_map(|(next_border, other_hash)| if other_hash == my_border_hash {
                            Some(TileConnection { id: self.id, my_border, next_tile: neighbor.id, next_border, flipped })
                        } else {
                            None
                        })
                        .collect::<Vec<_>>();

                        let number_of_shared_borders = matching_borders.len();

                    if number_of_shared_borders > 1 {
                        panic!(format!("Tile {} shares more than one border with tile {}", self.id, neighbor.id))
                    } else if number_of_shared_borders == 1 {
                        Some(matching_borders[0])
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        let mut connections = vec![];
        for h in self.data.iter().enumerate() {
            let (my_border, my_border_hash) = h;
            let mut connection = find(my_border, my_border_hash, true);
            connections.append(&mut connection);
        }

        // do everything again but flip it
        for h in self.data.iter().map(|h| Self::flip(*h)).enumerate() {
            let (my_border, my_border_hash) = h;
            let mut connection = find(my_border, &my_border_hash, false);
            connections.append(&mut connection);
        }

        connections
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for h in &self.data {
            print!("{}  ", h);
        }
        println!("");
    }
}

#[derive(Debug, Clone, Copy)]
struct TileConnection {
    id: u64,
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

fn find_next_tile(tile: u64, border: usize, connections: &Vec<TileConnection>) -> Option<&TileConnection> {
    connections.iter()
        .find(|c| c.next_tile == tile && c.next_border == border)
}

fn find_right_border(tile: u64, is_even: bool, connections: &Vec<TileConnection>) -> usize {
    connections.iter()
        .filter(|c| c.id == tile)
        .map(|c| c.my_border)
        // We are looking for the right border of a tile at the left border of
        // the image. Since it is a border tile, there are only three
        // neighbors. If the tile is not rotated, there will be two neighbors
        // in even directions (0 = up, 2 = down). the same is true if the tile
        // is rotated 180 degrees. If it is rotated by 90/270 degrees, the
        // border to the right will be odd.
        .filter(|b| b % 2 == if is_even { 0 } else { 1 })
        .next()
        .unwrap()
}

fn rotation_from_exit_right(exit: usize) -> usize {
    match exit {
        1 => 0,
        2 => 3,
        3 => 2,
        0 => 1,
        _ => panic!(format!("cannot map exit {} to any rotation", exit)),
    }
}

fn rotation_from_exit_down(exit: usize) -> usize {
    match exit {
        1 => 1,
        2 => 0,
        3 => 3,
        0 => 2,
        _ => panic!(format!("cannot map exit {} to any rotation", exit)),
    }
}

pub fn problem2() -> Result<(), ParseError> {
    let tiles = parse_input()?;

    let hashes = tiles.iter()
        .map(|t| t.hashes())
        .collect::<Vec<_>>();

    let relations = hashes.iter()
        .map(|h| (h.id, h.find_neighbors(&hashes)))
        .collect::<Vec<_>>();

    // Both the example and my input have a corner that can be considered "top
    // left" without rotation or flipping the image.
    // "Top left" is defined as the tile that has two neighbors and the
    // neighbors are to the right and below the top left tile, i.e.
    //   my_border = [1, 2]
    let top_left = &relations.iter()
        // find corners
        .filter(|r| r.1.len() == 2)
        // find corner with neighbors to the right (my_border == 1) and bottom
        // (my_border == 2) of the corner
        .filter(|r| {
            let my_borders = r.1.iter().map(|v| v.my_border).collect::<Vec<_>>();
            my_borders.contains(&1) && my_borders.contains(&2)
        })
        .next().unwrap();

    println!("top left: {:?}", top_left);
    let connections = relations.iter()
        .map(|v| &v.1)
        .cloned()
        .flatten()
        .collect::<Vec<_>>();

    let mut connections_map = HashMap::new();
    for c in &connections {
        connections_map.entry((c.id, c.my_border)).or_insert(c);
    }

    // construct the image based on the tile connections
    let size = (tiles.len() as f32).sqrt() as usize;

    let mut current_y_tile = top_left.0;
    let mut current_y_border = 2;
    let mut image = vec![];
    let mut is_x_border_even = false;
    let mut y_flip = false;
    let mut x_flip = false;
    let mut debug = true;
    for _y in 0..size {
        // println!("constructing line {}", y);

        println!("({},{})", if x_flip { "o" } else { "x" }, rotation_from_exit_down(current_y_border));

        // construct a line
        let mut current_tile = current_y_tile;
        let mut current_border = find_right_border(current_tile, is_x_border_even, &connections);
        let mut line = vec![(current_tile, rotation_from_exit_down(current_y_border), false, x_flip)];
        for _x in 0..size - 1 {
            // println!("#{}: {} on border {}", _x, current_tile, current_border);
            if debug {
                print!("{}  {} ", current_tile, current_border);
                // if current_tile == 1277 {
                //     debug = false;
                // }
            }
            if let Some(next) = find_next_tile(current_tile, current_border, &connections) {
                current_tile = next.id;
                current_border = (next.my_border + 2) % 4;
                if next.flipped {
                    y_flip = !y_flip;
                }
                if debug {
                    print!("({},{}) {}   ", if !next.flipped{ "o" } else { "x" }, rotation_from_exit_right(current_border), next.my_border);
                    // println!("{:?}", next);
                    // println!("cb {}, r {}, y {}, x {}", current_border, rotation_from_exit(current_border), y_flipped, x_flipped);
                }
                line.push((current_tile, rotation_from_exit_right(current_border), y_flip, false));
            }
        }
        println!("");
        image.push(line);

        if let Some(next_y) = find_next_tile(current_y_tile, current_y_border, &connections) {
            current_y_tile = next_y.id;
            current_y_border = (next_y.my_border + 2) % 4;
            if next_y.flipped {
                x_flip = !x_flip;
            }
            y_flip = x_flip;
            is_x_border_even = current_y_border % 2 != 0;

        }
    }

    // generate a hashmap for easier id based lookup of tiles
    let mut tile_map = HashMap::new();
    for tile in tiles {
        tile_map.entry(tile.id).or_insert(tile);
    }

    // reconstruct image
    let mut high_c = vec![];
    let mut printed_tiles = HashSet::new();
    for l in image.iter().take(12) {
        for y in 0..10 {
            let mut line = vec![];
            for (tile_id, rotation, y_flipped, x_flipped) in l.iter() {
                printed_tiles.insert((*tile_id, *rotation, *y_flipped, *x_flipped));
                let tile = tile_map.get(&tile_id).unwrap();
                let mut tile_line = tile.get_line(y, *rotation, *y_flipped, *x_flipped);
                line.append(&mut tile_line);
                line.push(' ');
            }
            high_c.push(line);
        }
        // high_c.push(vec![' '; 33]);
    }

    // for c in connections {
    //     println!("{} {:?}", c.id, c);
    // }

    for l in image.iter().take(5) {
        for e in l {
            print!("{} ", e.0);
        }
        println!("");
    }

    println!("Considered tiles: {:?}", printed_tiles);
    let image = Tile { id: 0, data: high_c };
    image.print();

    // for r in relations {
    //     println!("Tile {}", r.0);
    //     println!("My borders: {:?}", r.1.iter().map(|v| v.my_border).collect::<Vec<_>>());
    // }

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