use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;

mod tile;
mod tilehash;
mod tileconnection;

use super::utils::ParseError;
use tile::Tile;
use tileconnection::TileConnection;

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
        .map(|h| (h.id(), h.number_of_neighbors(&hashes)))
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

fn reconstruct_image(tiles: &Vec<Tile>, connections: &Vec<TileConnection>, top_left: u64) -> Tile {
    let size = (tiles.len() as f32).sqrt() as usize;

    let mut current_y_tile = top_left;
    let mut current_y_border = 2;
    let mut image = vec![];
    let mut is_x_border_even = false;
    let mut y_flip = false;
    let mut x_flip = false;
    for _y in 0..size {
        // construct a line
        let mut current_tile = current_y_tile;
        let mut current_border = find_right_border(current_tile, is_x_border_even, &connections);
        let mut line = vec![(current_tile, rotation_from_exit_down(current_y_border), false, x_flip)];
        for _x in 0..size - 1 {
            if let Some(next) = find_next_tile(current_tile, current_border, &connections) {
                current_tile = next.id;
                current_border = (next.my_border + 2) % 4;
                if next.flipped {
                    y_flip = !y_flip;
                }
                line.push((current_tile, rotation_from_exit_right(current_border), y_flip, false));
            }
        }
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
        tile_map.entry(tile.id()).or_insert(tile);
    }

    // reconstruct image
    let mut high_c = vec![];
    let mut printed_tiles = HashSet::new();
    for l in image.iter() {
        for y in 0..8 {
            let mut line = vec![];
            for (tile_id, rotation, y_flipped, x_flipped) in l.iter() {
                printed_tiles.insert((*tile_id, *rotation, *y_flipped, *x_flipped));
                let tile = tile_map.get(&tile_id).unwrap();
                let mut tile_line = tile.get_line_without_border(y, *rotation, *y_flipped, *x_flipped);
                line.append(&mut tile_line);
            }
            high_c.push(line);
        }
    }

    Tile::new(0, high_c)
}

fn get_monster() -> Vec<Vec<char>> {
    include_str!("./data/monster.txt")
        .lines()
        .map(|v| v.chars().collect())
        .collect()
}

type Coords = (usize, usize);
fn find_monster(image: &Tile, monster: &Vec<Vec<char>>) -> Vec<Coords> {
    let size_image = image.data.len();
    let height_monster = monster.len();
    let width_monster = monster[0].len();

    let mut positions = vec![];
    for y in 0..(size_image-height_monster) {
        for x in 0..(size_image-width_monster) {
            let mut found = true;
            for my in 0..height_monster {
                for mx in 0..width_monster {
                    if monster[my][mx] == '#' && image.data[y + my][x + mx] != '#' {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }

            if found {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn transform_and_find_monster(image: &Tile, monster: &Vec<Vec<char>>) -> Option<(Tile, Vec<Coords>)> {
    for r in 0..4 {
        for f in vec![true, false] {
            let transformed = image.transform(r, f, false);
            let positions = find_monster(&transformed, monster);
            if !positions.is_empty() {
                return Some((transformed, positions));
            }
        }
    }

    None
}

fn remove_monsters(image: Tile, monster: &Vec<Vec<char>>, positions: &Vec<Coords>) -> Tile {
    let height_monster = monster.len();
    let width_monster = monster[0].len();
    let mut image = image;

    for p in positions {
        for y in 0..height_monster {
            for x in 0..width_monster {
                if monster[y][x] == '#' {
                    image.data[p.1 + y][p.0 + x] = 'O';
                }
            }
        }
    }
    image
}

pub fn problem2() -> Result<(), ParseError> {
    let tiles = parse_input()?;

    let hashes = tiles.iter()
        .map(|t| t.hashes())
        .collect::<Vec<_>>();

    let relations = hashes.iter()
        .map(|h| (h.id(), h.find_neighbors(&hashes)))
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
    let image = reconstruct_image(&tiles, &connections, top_left.0);

    // rotate and look at image + flip for monsters
    let monster = get_monster();
    if let Some((transformed, monsters)) = transform_and_find_monster(&image, &monster) {
        let image_without_monsters = remove_monsters(transformed, &monster, &monsters);
        // image_without_monsters.print();
        let result = image_without_monsters.count_sea();
        println!("20/2: water roughness: {}", result);
    } else {
        println!("No monsters found!");
    }

    Ok(())
}
