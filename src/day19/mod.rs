use super::utils::ParseError;
use dynparser::{parse, rules_from_peg};

type Rules<'a> = Vec<&'a str>;
type Input<'a> = Vec<&'a str>;

fn parse_input() -> (Rules<'static>, Input<'static>) {
    let raw_input = include_str!("./data/input.txt");
    let rules_and_input = raw_input
        .split("\n\n")
        .collect::<Vec<_>>();

    let rules = rules_and_input[0]
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    let input = rules_and_input[1]
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    (rules, input)
}

fn convert_rules_to_PEG(rules: Rules) -> String {
    let mut rules = rules;

    // dynparser expects one rule called 'main' as an entry point
    // Since rule '0' corresponds to this in our grammar, we just define one
    // additional rule:
    rules.push("main = 0");

    // the PEG parser expects a newline at the end of the string defining the
    // grammar...
    rules.push("");

    let ortrta: String = rules.join("\n");

    let ortrta = ortrta.replace("|", "\n  /");
    let ortrta = ortrta.replace(":", " =");
    let ortrta = ortrta.replace("\"a\"", r#"'a'"#);
    let ortrta = ortrta.replace("\"b\"", r#"'b'"#);

    ortrta
}

pub fn problem1() -> Result<(), ParseError> {
    let (rules, input) = parse_input();


    let prepared_rules = convert_rules_to_PEG(rules);

    let peg_rules = rules_from_peg(&prepared_rules).unwrap();

    let result = input.iter()
        .map(|v| parse(v, &peg_rules).is_ok())
        .filter(|m| *m)
        .count();

    print!("19/1: # of successfully parsed input lines: {}", result);

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