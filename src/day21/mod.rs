use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(.*)\s\(contains\s(.*)\)$").unwrap();
        }

        let cap = RE.captures(s).ok_or(ParseError::new(&format!("Could not extract ingredients: {}", s)))?;
        let ingredients = cap[1].split(" ").map(|v| v.to_owned()).collect::<Vec<_>>();
        let allergens = cap[2].split(", ").map(|v| v.to_owned()).collect::<Vec<_>>();

        Ok(Self { ingredients, allergens })
    }
}

fn parse_input() -> Result<Vec<Food>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| Food::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn map_allergens_to_food(food: &Vec<Food>) -> (HashMap<&str, HashSet<&str>>, HashSet<&str>) {
    let mut all_ingredients = HashSet::new();
    let mut map: HashMap<&str, Vec<HashSet<&str>>> = HashMap::new();
    for f in food {
        for a in &f.allergens {
            for i in &f.ingredients {
                all_ingredients.insert(i.as_str());
            }
            map.entry(&a)
                .and_modify(|l| l.push(f.ingredients.iter().map(|v| v.as_str()).collect::<HashSet<_>>()))
                .or_insert(vec![f.ingredients.iter().map(|v| v.as_str()).collect::<HashSet<_>>()]);
        }
    }

    let mut allergen_to_food_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (allergen, ingredient_lists) in &map {
        let mut result = all_ingredients.clone();
        for v in ingredient_lists.iter() {
            result = result.intersection(v).cloned().collect::<HashSet<_>>();
        }
        allergen_to_food_map.entry(allergen).or_insert(result);
    }

    (allergen_to_food_map, all_ingredients)
}

pub fn problem1() -> Result<(), ParseError> {
    let food = parse_input()?;

    let (allergen_to_food_map, all_ingredients) = map_allergens_to_food(&food);

    let mut allergenic_ingredients = HashSet::new();
    for (_, ingredients) in &allergen_to_food_map {
        for ingredient in ingredients {
            allergenic_ingredients.insert(ingredient);
        }
    }

    let mut non_allergenic_ingredients = all_ingredients.clone();
    for ingredient in allergenic_ingredients {
        non_allergenic_ingredients.remove(ingredient);
    }

    let result = food.iter()
        .map(|f| f.ingredients.iter().filter(|i| non_allergenic_ingredients.contains(i.as_str())))
        .flatten()
        .count();

    println!("21/1: # of times all non-allergenic food appears: {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input()?;

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