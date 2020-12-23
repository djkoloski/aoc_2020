use std::{collections::{HashMap, HashSet}, str::FromStr};

use problem::{Problem, solve};

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[derive(Debug)]
enum ParseFoodError {
    NoAllergens,
}

impl FromStr for Food {
    type Err = ParseFoodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contains = s.find(" (contains ").ok_or(ParseFoodError::NoAllergens)?;
        let ingredients = s[0..contains].split(' ').map(|s| s.to_string()).collect();
        let allergens = s[contains + 11..s.len() - 1].split(", ").map(|s| s.to_string()).collect();

        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

struct Day21;
impl Problem for Day21 {
    type Input = Vec<Food>;
    type Part1Output = usize;
    type Part2Output = String;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut candidates = HashMap::new();
        for food in input.iter() {
            for ingredient in food.ingredients.iter() {
                for allergen in food.allergens.iter() {
                    candidates.entry(ingredient.clone()).or_insert(HashSet::new()).insert(allergen.clone());
                }
            }
        }

        for i in 0..input.len() {
            for j in (0..input.len()).filter(|&n| n != i) {
                let common_allergens = input[i].allergens.intersection(&input[j].allergens).collect::<Vec<_>>();
                for ingredient in input[i].ingredients.difference(&input[j].ingredients) {
                    for &allergen in common_allergens.iter() {
                        candidates.get_mut(ingredient).unwrap().remove(allergen);
                    }
                }
            }
        }

        let resolved = candidates.iter()
            .filter_map(|(k, v)| {
                if v.len() == 1 {
                    Some((k.to_string(), v.iter().next().unwrap().to_string()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (resolved_ingredient, resolved_allergen) in resolved.iter() {
            for (ingredient, candidates) in candidates.iter_mut() {
                if ingredient != resolved_ingredient {
                    candidates.remove(resolved_allergen);
                }
            }
        }

        let non_allergens = candidates.iter().filter_map(|(k, v)| if v.len() == 0 { Some(k) } else { None }).collect::<HashSet<_>>();

        Ok(input.iter().map(|food| food.ingredients.iter().filter(|i| non_allergens.contains(i)).count()).sum())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut candidates = HashMap::new();
        for food in input.iter() {
            for ingredient in food.ingredients.iter() {
                for allergen in food.allergens.iter() {
                    candidates.entry(ingredient.clone()).or_insert(HashSet::new()).insert(allergen.clone());
                }
            }
        }

        for i in 0..input.len() {
            for j in (0..input.len()).filter(|&n| n != i) {
                let common_allergens = input[i].allergens.intersection(&input[j].allergens).collect::<Vec<_>>();
                for ingredient in input[i].ingredients.difference(&input[j].ingredients) {
                    for &allergen in common_allergens.iter() {
                        candidates.get_mut(ingredient).unwrap().remove(allergen);
                    }
                }
            }
        }


        loop {
            let mut changed = false;
            let resolved = candidates.iter()
                .filter_map(|(k, v)| {
                    if v.len() == 1 {
                        Some((k.to_string(), v.iter().next().unwrap().to_string()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            for (resolved_ingredient, resolved_allergen) in resolved.iter() {
                for (ingredient, candidates) in candidates.iter_mut() {
                    if ingredient != resolved_ingredient {
                        changed = changed || candidates.remove(resolved_allergen);
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let mut resolved = candidates.iter()
            .filter_map(|(k, v)| {
                if v.len() == 1 {
                    Some((v.iter().next().unwrap().to_string(), k.to_string()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        resolved.sort();

        Ok(resolved.iter().fold(String::new(), |acc, (_, ingredient)| if acc == "" { ingredient.to_string() } else { format!("{},{}", acc, ingredient) }))
    }
}

fn main() {
    solve::<Day21>("input").unwrap();
}
