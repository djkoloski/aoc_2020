use std::{collections::HashSet, io, num::ParseIntError, str::FromStr};

use problem::{CSV, Input, Problem, solve};

struct TicketField {
    name: String,
    ranges: Vec<(u32, u32)>,
}

impl TicketField {
    fn matches(&self, value: u32) -> bool {
        self.ranges.iter().any(|&(lower, upper)| value >= lower && value <= upper)
    }
}

#[derive(Debug)]
enum ParseTicketFieldError {
    ParseIntError(ParseIntError),
    MissingColon,
    MissingDash,
}

impl From<ParseIntError> for ParseTicketFieldError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl FromStr for TicketField {
    type Err = ParseTicketFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(':');
        let name = pieces.next().unwrap().to_string();
        let ranges = pieces.next()
            .ok_or(ParseTicketFieldError::MissingColon)?
            .trim()
            .split(" or ")
            .map(|p| {
                let mut pieces = p.split('-');
                let lower = pieces.next().unwrap().parse()?;
                let upper = pieces.next().ok_or(ParseTicketFieldError::MissingDash)?.parse()?;
                Ok((lower, upper))
            })
            .collect::<Result<_, ParseTicketFieldError>>()?;
        Ok(Self {
            name,
            ranges,
        })
    }
}

type Ticket = CSV<u32>;

struct Info {
    fields: Vec<TicketField>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug)]
enum ParseInfoError {
    IoError(io::Error),
    ParseTicketFieldError(ParseTicketFieldError),
    ParseIntError(ParseIntError),
    UnexpectedEndOfInput,
    MissingYourTicket,
    MissingNearbyTickets,
}

impl From<io::Error> for ParseInfoError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseTicketFieldError> for ParseInfoError {
    fn from(e: ParseTicketFieldError) -> Self {
        Self::ParseTicketFieldError(e)
    }
}

impl From<ParseIntError> for ParseInfoError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl Input for Info {
    type Error = ParseInfoError;

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();

        let mut fields = Vec::new();

        loop {
            let next = lines.next().ok_or(ParseInfoError::UnexpectedEndOfInput)??;
            if next == "" {
                break;
            }
            fields.push(next.parse()?);
        }

        if lines.next().ok_or(ParseInfoError::UnexpectedEndOfInput)?? != "your ticket:" {
            return Err(ParseInfoError::MissingYourTicket);
        }

        let your_ticket = lines.next().ok_or(ParseInfoError::UnexpectedEndOfInput)??.parse()?;

        lines.next();

        if lines.next().ok_or(ParseInfoError::UnexpectedEndOfInput)?? != "nearby tickets:" {
            return Err(ParseInfoError::MissingNearbyTickets);
        }

        let mut nearby_tickets = Vec::new();

        while let Some(next) = lines.next() {
            nearby_tickets.push(next?.parse()?);
        }

        Ok(Self {
            fields,
            your_ticket,
            nearby_tickets,
        })
    }
}

struct Day16;
impl Problem for Day16 {
    type Input = Info;
    type Part1Output = u32;
    type Part2Output = u64;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        Ok(
            input.nearby_tickets.iter()
                .map(|t| {
                    t.values.iter()
                        .filter(|&v| {
                            !input.fields.iter()
                                .any(|f| f.matches(*v))
                        })
                        .sum::<u32>()
                })
                .sum()
        )
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut candidates = HashSet::new();
        for i in 0..input.fields.len() {
            candidates.insert(i);
        }
        let mut field_candidates = Vec::new();
        for _ in 0..input.fields.len() {
            field_candidates.push(candidates.clone());
        }

        for ticket in input.nearby_tickets.iter()
            .filter(|&t| {
                t.values.iter().all(|&v| {
                    input.fields.iter().any(|f| f.matches(v))
                })
            })
        {
            for (i, &v) in ticket.values.iter().enumerate() {
                field_candidates[i] = field_candidates[i].iter()
                    .cloned()
                    .filter(|&c| input.fields[c].matches(v))
                    .collect();
            }
        }

        let mut fields = vec![0; input.fields.len()];

        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..field_candidates.len() {
                if field_candidates[i].len() == 1 {
                    let value = *field_candidates[i].iter().next().unwrap();
                    fields[i] = value;
                    field_candidates[i].clear();
                    for j in 0..field_candidates.len() {
                        field_candidates[j].remove(&value);
                    }
                    changed = true;
                }
            }
        }

        let mut total = 1;
        for (i, v) in input.your_ticket.values.iter().enumerate() {
            if input.fields[fields[i]].name.starts_with("departure") {
                total *= *v as u64;
            }
        }

        Ok(total)
    }
}

fn main() {
    solve::<Day16>("input").unwrap();
}
