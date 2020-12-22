use std::collections::VecDeque;
use super::utils::ParseError;

type Deck = VecDeque<u64>;

fn parse_deck(s: &str) -> Deck {
    s.lines()
        .skip(1)
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn parse_input() -> Vec<Deck> {
    let input = include_str!("./data/example.txt");
    input
        .split("\n\n")
        .filter(|v| *v != "")
        .map(|v| parse_deck(v))
        .collect::<Vec<_>>()
}

fn turn(players: Vec<Deck>) -> Result<Vec<Deck>, ParseError> {
    let mut tops = vec![];
    let mut players = players;
    for p in &mut players {
        let v = p.pop_front();
        tops.push(v);
    }

    let top = tops.iter()
        .enumerate()
        .filter_map(|(i, v)| if v.is_some() {
            Some((i, v.unwrap()))
        } else {
            None
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .ok_or(ParseError::new("No player has cards left"))?;

    let top_player = top.0;
    let mut tops_sorted = tops.into_iter().filter_map(|v| v).collect::<Vec<_>>();
    tops_sorted.sort_by(|a, b| b.cmp(&a));

    let mut players = players;
    for t in tops_sorted {
        players[top_player].push_back(t);
    }

    Ok(players)
}

fn print_decks(players: &Vec<Deck>) {
    for (i, p) in players.iter().enumerate() {
        println!("Player {}: {:?}", i, p);
    }
}

pub fn problem1() -> Result<(), ParseError> {
    let mut decks = parse_input();

    loop {
        decks = turn(decks)?;
        if decks.iter().any(|v| v.is_empty()) {
            break;
        }
    }

    let winner = decks.iter().find(|d| !d.is_empty()).unwrap();
    let score: u64 = winner.iter().rev().enumerate()
        .map(|(i, v)| (i + 1, v))
        .map(|(i, v)| (i as u64) * v)
        .sum();

    println!("22/1: score of winner's deck: {}", score);

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