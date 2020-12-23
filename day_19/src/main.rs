use std::{collections::HashMap, io, num::ParseIntError, str::FromStr};
use problem::{solve, Problem, ProblemInput};

#[derive(Clone)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    Alternate(Vec<usize>, Vec<usize>),
}

impl Rule {
    fn matches<'a>(&self, rules: &HashMap<usize, Rule>, s: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        match self {
            &Rule::Literal(c) => {
                if s.chars().next() == Some(c) {
                    results.push(&s[1..]);
                }
            },
            Rule::Sequence(seq) => {
                let mut current_matches = vec![s];
                for rule in seq.iter() {
                    let mut next_matches = Vec::new();
                    for s in current_matches {
                        next_matches.append(&mut rules[rule].matches(rules, s));
                    }
                    current_matches = next_matches;
                }
                results.append(&mut current_matches);
            },
            Rule::Alternate(seq_a, seq_b) => {
                for &seq in [seq_a, seq_b].iter() {
                    let mut current_matches = vec![s];
                    for rule in seq.iter() {
                        let mut next_matches = Vec::new();
                        for s in current_matches {
                            next_matches.append(&mut rules[rule].matches(rules, s));
                        }
                        current_matches = next_matches;
                    }
                    results.append(&mut current_matches);
                }
            }
        }
        results
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 3 && s.chars().nth(0).unwrap() == '"' && s.chars().nth(2).unwrap() == '"' {
            Ok(Rule::Literal(s.chars().nth(1).unwrap()))
        } else if let Some(bar_pos) = s.find(" | ") {
            Ok(Rule::Alternate(
                s[..bar_pos].split(' ').map(|p| p.parse()).collect::<Result<_, _>>()?,
                s[bar_pos + 3..].split(' ').map(|p| p.parse()).collect::<Result<_, _>>()?,
            ))
        } else {
            Ok(Rule::Sequence(
                s.split(' ').map(|p| p.parse()).collect::<Result<_, _>>()?
            ))
        }
    }
}

struct Input {
    rules: HashMap<usize, Rule>,
    strings: Vec<String>,
}

#[derive(Debug)]
enum ParseInputError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    MissingRuleId,
    MissingRuleDef,
}

impl From<io::Error> for ParseInputError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for ParseInputError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl ProblemInput for Input {
    type Error = ParseInputError;

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();

        let mut rules = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line?;
            if line.len() == 0 {
                break;
            }
            let mut pieces = line.split(": ");
            let index = pieces.next().ok_or(ParseInputError::MissingRuleId)?.parse()?;
            let rule = pieces.next().ok_or(ParseInputError::MissingRuleDef)?.parse()?;
            rules.insert(index, rule);
        }

        let mut strings = Vec::new();
        while let Some(line) = lines.next() {
            strings.push(line?.to_string());
        }

        Ok(Self {
            rules,
            strings,
        })
    }
}

struct Day19;
impl Problem for Day19 {
    type Input = Input;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        Ok(input.strings.iter().map(|s| input.rules[&0].matches(&input.rules, s.as_str())).filter(|matches| matches.contains(&"")).count())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut rules = input.rules.clone();
        rules.insert(8, Rule::Alternate(vec![42], vec![42, 8]));
        rules.insert(11, Rule::Alternate(vec![42, 31], vec![42, 11, 31]));
        Ok(input.strings.iter().map(|s| rules[&0].matches(&rules, s.as_str())).filter(|matches| matches.contains(&"")).count())
    }
}

fn main() {
    solve::<Day19>("input").unwrap();
}