use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};
use problem::{Problem, solve};

#[derive(Clone, Eq, Hash, PartialEq)]
enum Modifier {
    Light,
    Dark,
    Bright,
    Muted,
    Shiny,
    Vibrant,
    Faded,
    Dotted,
    Pale,
    Striped,
    Posh,
    Wavy,
    Drab,
    Clear,
    Dull,
    Plaid,
    Mirrored,
    Dim,
}

#[derive(Debug)]
struct ParseModifierError(String);

impl FromStr for Modifier {
    type Err = ParseModifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Modifier::*;
         Ok(match s {
             "light" => Light,
             "dark" => Dark,
             "bright" => Bright,
             "muted" => Muted,
             "shiny" => Shiny,
             "vibrant" => Vibrant,
             "faded" => Faded,
             "dotted" => Dotted,
             "pale" => Pale,
             "striped" => Striped,
             "posh" => Posh,
             "wavy" => Wavy,
             "drab" => Drab,
             "clear" => Clear,
             "dull" => Dull,
             "plaid" => Plaid,
             "mirrored" => Mirrored,
             "dim" => Dim,
             s => return Err(ParseModifierError(s.to_string())),
         })
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Color {
    Red,
    Orange,
    White,
    Yellow,
    Gold,
    Olive,
    Plum,
    Blue,
    Black,
    Turquoise,
    Cyan,
    Teal,
    Tan,
    Tomato,
    Coral,
    Bronze,
    Purple,
    Crimson,
    Beige,
    Salmon,
    Maroon,
    Lavender,
    Lime,
    Indigo,
    Chartreuse,
    Magenta,
    Silver,
    Violet,
    Brown,
    Aqua,
    Green,
    Gray,
    Fuchsia,
}

#[derive(Debug)]
struct ParseColorError(String);

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Color::*;
         Ok(match s {
             "red" => Red,
             "orange" => Orange,
             "white" => White,
             "yellow" => Yellow,
             "gold" => Gold,
             "olive" => Olive,
             "plum" => Plum,
             "blue" => Blue,
             "black" => Black,
             "turquoise" => Turquoise,
             "cyan" => Cyan,
             "teal" => Teal,
             "tan" => Tan,
             "tomato" => Tomato,
             "coral" => Coral,
             "bronze" => Bronze,
             "purple" => Purple,
             "crimson" => Crimson,
             "beige" => Beige,
             "salmon" => Salmon,
             "maroon" => Maroon,
             "lavender" => Lavender,
             "lime" => Lime,
             "indigo" => Indigo,
             "chartreuse" => Chartreuse,
             "magenta" => Magenta,
             "silver" => Silver,
             "violet" => Violet,
             "brown" => Brown,
             "aqua" => Aqua,
             "green" => Green,
             "gray" => Gray,
             "fuchsia" => Fuchsia,
             s => return Err(ParseColorError(s.to_string())),
         })
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Bag {
    pub modifier: Modifier,
    pub color: Color,
}

#[derive(Debug)]
enum ParseBagError {
    InvalidFormat(String),
    ParseModifierError(ParseModifierError),
    ParseColorError(ParseColorError),
}

impl From<ParseModifierError> for ParseBagError {
    fn from(e: ParseModifierError) -> Self {
        Self::ParseModifierError(e)
    }
}

impl From<ParseColorError> for ParseBagError {
    fn from(e: ParseColorError) -> Self {
        Self::ParseColorError(e)
    }
}

impl FromStr for Bag {
    type Err = ParseBagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        Ok(Bag {
            modifier: pieces.next().ok_or(ParseBagError::InvalidFormat(s.to_string()))?.parse()?,
            color: pieces.next().ok_or(ParseBagError::InvalidFormat(s.to_string()))?.parse()?,
        })
    }
}

struct Rule {
    pub outer: Bag,
    pub inner: Vec<(usize, Bag)>,
}

#[derive(Debug)]
enum ParseRuleError {
    InvalidFormat,
    ParseIntError(ParseIntError),
    ParseBagError(ParseBagError),
}

impl From<ParseIntError> for ParseRuleError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseBagError> for ParseRuleError {
    fn from(e: ParseBagError) -> Self {
        Self::ParseBagError(e)
    }
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split("bags contain");
        let outer = pieces.next().ok_or(ParseRuleError::InvalidFormat)?.parse()?;

        let inner = pieces.next()
            .ok_or(ParseRuleError::InvalidFormat)?
            .split("bag")
            .map(|s| s.trim())
            .filter_map(|s| {
                if s == "no other" || s == "." || s == "s." {
                    None
                } else if s.starts_with(", ") {
                    Some(&s[2..])
                } else if s.starts_with("s, ") {
                    Some(&s[3..])
                } else {
                    Some(s)
                }
            })
            .map(|s| {
                let (amount, bag) = s.split_at(s.find(' ').unwrap());
                Ok((
                    amount.parse()?,
                    bag[1..].parse()?,
                ))
            })
            .collect::<Result<_, ParseRuleError>>()?;

        Ok(Self {
            outer,
            inner,
        })
    }
}

struct Day7;
impl Problem for Day7 {
    type Input = Rule;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Vec<Self::Input>) -> Result<Self::Part1Output, Self::Error> {
        let mut graph = HashMap::new();

        for i in input {
            if !graph.contains_key(&i.outer) {
                graph.insert(i.outer.clone(), Vec::new());
            }

            for inner in i.inner.iter() {
                graph.entry(inner.1.clone()).or_insert(Vec::new()).push(i.outer.clone());
            }
        }

        let mut visited = HashSet::new();
        let mut frontier = graph.get(&Bag { modifier: Modifier::Shiny, color: Color::Gold }).unwrap().clone();

        while let Some(next) = frontier.pop() {
            if visited.insert(next.clone()) {
                for outer in graph.get(&next).unwrap() {
                    frontier.push(outer.clone());
                }
            }
        }

        Ok(visited.len())
    }

    fn part_2(input: &Vec<Self::Input>) -> Result<Self::Part1Output, Self::Error> {
        let mut graph = HashMap::new();
        for i in input {
            graph.insert(i.outer.clone(), i.inner.clone());
        }

        let mut frontier = graph.get(&Bag { modifier: Modifier::Shiny, color: Color::Gold }).unwrap().clone();
        let mut total = 0;
        while let Some(next) = frontier.pop() {
            total += next.0;
            for (amount, bag) in graph.get(&next.1).unwrap() {
                frontier.push((amount * next.0, bag.clone()))
            }
        }

        Ok(total)
    }
}

fn main() {
    solve::<Day7>("input").unwrap();
}