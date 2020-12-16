use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn is_valid(&self, i: u32) -> bool {
        self.min <= i && i <= self.max
    }
}

#[derive(Debug)]
struct Rule {
    description: String,
    ranges: Vec<Range>,
}

impl Rule {
    fn is_valid(&self, i: u32) -> bool {
        self.ranges.iter().any(|r| r.is_valid(i))
    }
}

type Ticket = Vec<u32>;

#[derive(Debug)]
struct Puzzle {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn read_rule(s: &str) -> Rule {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(.+?): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let cap = RE.captures(s).unwrap();
    let description = cap[1].to_string();

    let mut ranges = vec![];

    let min = cap[2].parse::<u32>().unwrap();
    let max = cap[3].parse::<u32>().unwrap();
    ranges.push(Range { min, max });

    let min = cap[4].parse::<u32>().unwrap();
    let max = cap[5].parse::<u32>().unwrap();
    ranges.push(Range { min, max });

    Rule { description, ranges }
}

fn read_ticket(s: &str) -> Ticket {
    s.split(',')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_input() -> Puzzle {
    let input = include_str!("./data/input.txt");
    let puzzle = input
        .split("\n\n")
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    let rules = puzzle[0].lines()
        .filter(|v| !v.is_empty())
        .map(|l| read_rule(l))
        .collect::<Vec<_>>();

    let my_ticket = puzzle[1].lines()
        .skip(1)
        .map(|t| read_ticket(t))
        .next().unwrap();

    let nearby_tickets = puzzle[2].lines()
        .skip(1)
        .filter(|v| !v.is_empty())
        .map(|l| read_ticket(l))
        .collect::<Vec<_>>();

    Puzzle { rules, my_ticket, nearby_tickets }
}

fn is_valid_for_some_field(rules: &Vec<Rule>, n: u32) -> bool {
    rules.iter()
        .any(|r| r.is_valid(n))
}

fn find_invalid_fields(rules: &Vec<Rule>, ticket: &Ticket) -> Vec<u32> {
    ticket.iter()
        .filter(|n| !is_valid_for_some_field(rules, **n))
        .cloned()
        .collect::<Vec<_>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input();

    // println!("{:?}", input);

    let mut invalid_fields = vec![];
    for t in &input.nearby_tickets {
        let mut invalid_fields_for_ticket = find_invalid_fields(&input.rules, t);
        invalid_fields.append(&mut invalid_fields_for_ticket);
    }

    // println!("{:?}", invalid_fields);
    // let foo = find_invalid_fields(&input.rules, &input.nearby_tickets[2]);
    // println!("first: {:?}", foo);
    let result: u32 = invalid_fields.iter().sum();
    println!("16/1: sum of invalid fields is {}", result);

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