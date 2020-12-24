use std::{collections::{HashSet, VecDeque}, io, num};
use problem::{Problem, ProblemInput, solve};

struct Input {
    player_1: Vec<u32>,
    player_2: Vec<u32>,
}

#[derive(Debug)]
enum ParseInputError {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    UnexpectedEndOfInput,
    ExpectedPlayer1,
    ExpectedPlayer2,
}

impl From<io::Error> for ParseInputError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<num::ParseIntError> for ParseInputError {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl ProblemInput for Input {
    type Error = ParseInputError;

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();

        if lines.next().ok_or(ParseInputError::UnexpectedEndOfInput)?? != "Player 1:" {
            return Err(ParseInputError::ExpectedPlayer1);
        }

        let mut player_1 = Vec::new();
        loop {
            let next = lines.next().ok_or(ParseInputError::UnexpectedEndOfInput)??;
            if next != "" {
                player_1.push(next.parse()?);
            } else {
                break;
            }
        }

        if lines.next().ok_or(ParseInputError::UnexpectedEndOfInput)?? != "Player 2:" {
            return Err(ParseInputError::ExpectedPlayer2);
        }

        let mut player_2 = Vec::new();
        loop {
            if let Some(next) = lines.next() {
                player_2.push(next?.parse()?);
            } else {
                break;
            }
        }

        Ok(Self {
            player_1,
            player_2,
        })
    }
}

enum RecursiveCombatWinner {
    Player1(VecDeque<u32>),
    Player2(VecDeque<u32>),
}

// returns "did player 1 win"
fn recursive_combat(mut player_1: VecDeque<u32>, mut player_2: VecDeque<u32>) -> RecursiveCombatWinner {
    let mut previous_states = HashSet::new();

    while player_1.len() > 0 && player_2.len() > 0 {
        if !previous_states.insert((player_1.clone(), player_2.clone())) {
            return RecursiveCombatWinner::Player1(player_1);
        } else {
            let card_1 = player_1.pop_front().unwrap();
            let card_2 = player_2.pop_front().unwrap();

            if card_1 as usize <= player_1.len() && card_2 as usize <= player_2.len() {
                let player_1_deck = player_1.iter().take(card_1 as usize).cloned().collect();
                let player_2_deck = player_2.iter().take(card_2 as usize).cloned().collect();
                match recursive_combat(player_1_deck, player_2_deck) {
                    RecursiveCombatWinner::Player1(_) => {
                        player_1.push_back(card_1);
                        player_1.push_back(card_2);
                    },
                    RecursiveCombatWinner::Player2(_) => {
                        player_2.push_back(card_2);
                        player_2.push_back(card_1);
                    },
                }
            } else if card_1 > card_2 {
                player_1.push_back(card_1);
                player_1.push_back(card_2);
            } else {
                player_2.push_back(card_2);
                player_2.push_back(card_1);
            }
        }
    }

    if player_1.len() > 0 {
        RecursiveCombatWinner::Player1(player_1)
    } else {
        RecursiveCombatWinner::Player2(player_2)
    }
}

struct Day22;
impl Problem for Day22 {
    type Input = Input;
    type Part1Output = u32;
    type Part2Output = u32;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut player_1 = input.player_1.iter().cloned().collect::<VecDeque<_>>();
        let mut player_2 = input.player_2.iter().cloned().collect::<VecDeque<_>>();

        while player_1.len() > 0 && player_2.len() > 0 {
            let card_1 = player_1.pop_front().unwrap();
            let card_2 = player_2.pop_front().unwrap();

            if card_1 > card_2 {
                player_1.push_back(card_1);
                player_1.push_back(card_2);
            } else {
                player_2.push_back(card_2);
                player_2.push_back(card_1);
            }
        }

        let winner = if player_1.len() > 0 { player_1 } else { player_2 };
        Ok(winner.iter().rev().enumerate().map(|(i, v)| (i as u32 + 1) * v).sum())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let winner = match recursive_combat(input.player_1.iter().cloned().collect(), input.player_2.iter().cloned().collect()) {
            RecursiveCombatWinner::Player1(deck) => deck,
            RecursiveCombatWinner::Player2(deck) => deck,
        };
        Ok(winner.iter().rev().enumerate().map(|(i, v)| (i as u32 + 1) * v).sum())
    }
}

fn main() {
    solve::<Day22>("input").unwrap();
}
