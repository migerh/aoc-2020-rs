use super::utils::ParseError;

enum Floor {
    Tree,
    Free,
}

fn parse_line(line: &str) -> Vec<Floor> {
    line.chars().map(|v| match v {
        '#' => Floor::Tree,
        _ => Floor::Free,
    })
    .collect::<Vec<_>>()
}

fn parse_input() -> Vec<Vec<Floor>> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| parse_line(v))
        .collect::<Vec<_>>()
}

fn count_trees_on_path(map: &Vec<Vec<Floor>>, slope: &(usize, usize)) -> u64 {
    let mut coords = (0, 0);
    let mut number_of_trees = 0;
    while coords.1 < map.len() - 1 {
        coords.0 = (coords.0 + slope.0) % map[0].len();
        coords.1 += slope.1;

        number_of_trees += match map[coords.1][coords.0] {
            Floor::Tree => 1,
            Floor::Free => 0,
        };
    }

    number_of_trees
}

pub fn problem1() -> Result<u64, ParseError> {
    let map = parse_input();

    let number_of_trees = count_trees_on_path(&map, &(3, 1));
    println!("3/1: # of trees: {}", number_of_trees);

    Ok(number_of_trees)
}

pub fn problem2() -> Result<u64, ParseError> {
    let map = parse_input();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result: u64 = slopes.iter()
        .map(|slope| count_trees_on_path(&map, slope))
        .product();

    println!("3/2: product of # of trees on all slopes considered: {}", result);

    Ok(result)
}
