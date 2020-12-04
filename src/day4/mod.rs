use super::utils::ParseError;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn find_value_for_key<'a>(input: &str, key: &str) -> Option<String> {
    let values = input.split(' ')
        .map(|kv| kv.split(':').collect::<Vec<_>>())
        .filter(|kv| kv[0] == key)
        .collect::<Vec<_>>();

    if values.is_empty() {
        None
    } else {
        Some(values[0][1].to_owned())
    }
}

impl From<&String> for Passport {
    fn from(input: &String) -> Self {
        let birth_year = find_value_for_key(input, "byr");
        let issue_year = find_value_for_key(input, "iyr");
        let expiration_year = find_value_for_key(input, "eyr");
        let height = find_value_for_key(input, "hgt");
        let hair_color = find_value_for_key(input, "hcl");
        let eye_color = find_value_for_key(input, "ecl");
        let passport_id = find_value_for_key(input, "pid");
        let country_id = find_value_for_key(input, "cid");

        Self { birth_year, issue_year, expiration_year, height, hair_color, eye_color, passport_id, country_id }
    }
}

impl Passport {
    fn is_valid(self: &Self) -> bool {
        self.birth_year.is_some() &&
        self.issue_year.is_some() &&
        self.expiration_year.is_some() &&
        self.height.is_some() &&
        self.hair_color.is_some() &&
        self.eye_color.is_some() &&
        self.passport_id.is_some()
    }

    fn is_valid_for_2(self: &Self) -> Result<bool, ParseError> {
        if !self.is_valid() {
            return Ok(false);
        }

        if self.birth_year.is_some() {
            let year = self.birth_year.as_ref().unwrap().parse::<i32>()?;
            if year < 1920 || year > 2002 {
                return Ok(false);
            }
        }

        if self.issue_year.is_some() {
            let year = self.issue_year.as_ref().unwrap().parse::<i32>()?;
            if year < 2010 || year > 2020 {
                return Ok(false);
            }
        }

        if self.expiration_year.is_some() {
            let year = self.expiration_year.as_ref().unwrap().parse::<i32>()?;
            if year < 2020 || year > 2030 {
                return Ok(false);
            }
        }

        if self.height.is_some() {
            lazy_static!{
                static ref RE: Regex = Regex::new(r"(\d*)(cm|in)").unwrap();
            }
            if !RE.is_match(&self.height.as_ref().unwrap()) {
                return Ok(false);
            }
            let cap = RE.captures(&self.height.as_ref().unwrap()).unwrap();
            let size = cap[1].parse::<u32>()?;
            let unit = cap[2].to_string();

            if unit == "cm" && (size < 150 || size > 193) {
                return Ok(false);
            } else if unit == "in" && (size < 59 || size > 76) {
                return Ok(false);
            } else if unit != "cm" && unit != "in" {
                return Ok(false);
            }
        }

        if self.hair_color.is_some() {
            lazy_static!{
                static ref RE: Regex = Regex::new(r"^\#([0-9a-f]{6})$").unwrap();
            }

            if !RE.is_match(self.hair_color.as_ref().unwrap()) {
                return Ok(false)
            }
        }

        if self.eye_color.is_some() {
            lazy_static!{
                static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            }

            if !RE.is_match(self.eye_color.as_ref().unwrap()) {
                return Ok(false)
            }
        }

        if self.passport_id.is_some() {
            lazy_static!{
                static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }

            if !RE.is_match(self.passport_id.as_ref().unwrap()) {
                return Ok(false)
            }
        }

        Ok(true)
    }
}

fn parse_input() -> Vec<Passport> {
    let input = include_str!("./data/input.txt");

    let mut raw_passports = vec![];
    let mut passport_input = vec![];
    for line in input.lines() {
        if line.is_empty() {
            raw_passports.push(passport_input.join(" "));
            passport_input = vec![];
        } else {
            passport_input.push(line);
        }
    }
    raw_passports.push(passport_input.join(" "));

    raw_passports.iter().map(|v| Passport::from(v)).collect::<Vec<_>>()
}

pub fn problem1() -> Result<usize, ParseError> {
    let input = parse_input();

    let solution = input.iter()
        .filter(|p| p.is_valid())
        .count();

    println!("4/1: # of 'valid' passports: {}", solution);

    Ok(solution)
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input();

    let solution = input.iter()
        .map(|p| p.is_valid_for_2())
        .collect::<Result<Vec<_>, ParseError>>()?
        .iter()
        .filter(|&&b| b)
        .count();

    println!("4/2: # of 'valid' passports: {}", solution);
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