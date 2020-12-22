use std::collections::VecDeque;
use super::utils::ParseError;

type Deck = VecDeque<u64>;

#[derive(Clone)]
struct Game {
    player: Vec<Deck>,
    winner: Option<usize>,
}

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

fn turn(mut game: Game, previous_games: &mut Vec<Game>, recurse: bool) -> Game {
    println!("Player 1's deck: {:?}", game.player[0]);
    println!("Player 2's deck: {:?}", game.player[1]);

    let t1 = game.player[0].pop_front();
    let t2 = game.player[1].pop_front();

    if t1.is_none() {
        game.player[1].push_front(t2.unwrap());
        game.winner = Some(1);
        return game;
    }

    if t2.is_none() {
        game.player[0].push_front(t1.unwrap());
        game.winner = Some(0);
        return game;
    }

    let t1 = t1.unwrap();
    let t2 = t2.unwrap();
    let mut winner = 0;

    println!("Player 1 plays: {}", t1);
    println!("Player 2 plays: {}", t2);

    // determine sub game
    if recurse && t1 <= game.player[0].len() as u64 && t2 <= game.player[1].len() as u64 {
        previous_games.push(game.clone());
        let rg = play_game(game.clone(), previous_games, recurse);
        winner = rg.winner.unwrap();
        previous_games.pop();
    } else {
        if t2 > t1 {
            winner = 1;
        }
    }

    if winner == 0 {
        game.player[0].push_back(t1);
        game.player[0].push_back(t2);
    } else {
        game.player[1].push_back(t2);
        game.player[1].push_back(t1);
    }

    return game;
}

fn play_game(mut game: Game, previous_games: &mut Vec<Game>, recurse: bool) -> Game {
    let mut round = 0;
    loop {
        round += 1;
        println!("-- Round {} (Game {}) --", round, previous_games.len() + 1);
        game = turn(game, previous_games, recurse);
        if game.winner.is_some() {
            return game;
        }
    }
}

fn print_decks(players: &Vec<Deck>) {
    for (i, p) in players.iter().enumerate() {
        println!("Player {}: {:?}", i, p);
    }
}

pub fn problem1() -> Result<(), ParseError> {
    let decks = parse_input();
    let mut game = Game { player: decks, winner: None };

    loop {
        game = turn(game, &mut vec![], false);
        if game.winner.is_some() {
            break;
        }
    }

    let winner = game.winner.unwrap();
    let score: u64 = game.player[winner].iter().rev().enumerate()
        .map(|(i, v)| (i + 1, v))
        .map(|(i, v)| (i as u64) * v)
        .sum();

    println!("22/1: score of winner's deck: {}", score);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let decks = parse_input();
    let mut game = Game { player: decks, winner: None };

    game = play_game(game, &mut vec![], true);

    let winner = game.winner.unwrap();
    let score: u64 = game.player[winner].iter().rev().enumerate()
        .map(|(i, v)| (i + 1, v))
        .map(|(i, v)| (i as u64) * v)
        .sum();

    println!("22/2: score of winner's deck: {}", score);

    Ok(())
}
