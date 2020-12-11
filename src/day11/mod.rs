use std::collections::HashMap;
use super::utils::ParseError;

fn parse_input() -> Vec<Vec<char>> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

type Coords = (i32, i32);
type World = HashMap<Coords, char>;

fn map_size(input: &Vec<Vec<char>>) -> Result<Coords, ParseError> {
    let height = input.len();
    let width = input.last().ok_or(ParseError::new("Empty map."))?.len();

    Ok((height as i32, width as i32))
}

fn generate_world(input: Vec<Vec<char>>) -> World {
    let mut map = HashMap::new();

    input.into_iter()
        .enumerate()
        .for_each(|(row, line)| {
            line.into_iter()
                .enumerate()
                .for_each(|(col, char)| {
                    map.entry((row as i32, col as i32)).or_insert(char);
                })
        });

    map
}

fn count_occupied_neighbors(map: &World, coords: &Coords) -> usize {
    let mut count = 0;

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }

            if let Some(status) = map.get(&(coords.0 + i, coords.1 + j)) {
                if status == &'#' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn tick(map: World) -> World {
    let mut new_world = HashMap::new();

    for (coords, status) in &map {
        let occupied_neighbors = count_occupied_neighbors(&map, &coords);

        if status == &'L' && occupied_neighbors == 0 {
            new_world.entry(*coords).or_insert('#');
        } else if status == &'#' && occupied_neighbors >= 4 {
            new_world.entry(*coords).or_insert('L');
        } else {
            new_world.entry(*coords).or_insert(*status);
        }
    }

    new_world
}

fn count_occupied_seats(map: &World) -> usize {
    map.iter()
        .filter(|(_, &v)| v == '#')
        .count()
}

fn print_world(world: &World, size: &Coords) {
    for row in 0..size.0 {
        for col in 0..size.1 {
            print!("{}", world.get(&(row, col)).unwrap());
        }
        println!("");
    }
    println!("");
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input();
    let size = map_size(&input)?;
    let mut old_world = generate_world(input);

    // print_world(&old_world, &size);

    let mut last_count = 0;
    loop {
        let new_world = tick(old_world.clone());
        // print_world(&new_world, &size);

        let occupied = count_occupied_seats(&new_world);
        if last_count == occupied {
            break;
        }
        old_world = new_world;
        last_count = occupied;
    }

    println!("11/1: # of occupied seats: {}", count_occupied_seats(&old_world));

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