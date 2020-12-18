use std::str::FromStr;
use problem::{Problem, solve};

#[derive(Debug)]
enum Token {
    Int(u64),
    Plus,
    Asterisk,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
enum ParseError {
    InvalidChar(char),
}

struct Expression {
    tokens: Vec<Token>,
}

enum Operator {
    Add,
    Mul,
    LeftParen,
}

#[derive(Debug)]
enum EvaluationError {
    MissingLHS,
    MissingRHS,
    MismatchedParens,
    InvalidExpression,
}

impl Expression {
    fn add(outputs: &mut Vec<u64>) -> Result<u64, EvaluationError> {
        let lhs = outputs.pop().ok_or(EvaluationError::MissingLHS)?;
        let rhs = outputs.pop().ok_or(EvaluationError::MissingRHS)?;
        Ok(lhs + rhs)
    }

    fn mul(outputs: &mut Vec<u64>) -> Result<u64, EvaluationError> {
        let lhs = outputs.pop().ok_or(EvaluationError::MissingLHS)?;
        let rhs = outputs.pop().ok_or(EvaluationError::MissingRHS)?;
        Ok(lhs * rhs)
    }

    fn reduce(outputs: &mut Vec<u64>, operators: &mut Vec<Operator>) -> Result<(), EvaluationError> {
        while operators.len() > 0 {
            let result = match operators[operators.len() - 1] {
                Operator::Add => Self::add(outputs)?,
                Operator::Mul => Self::mul(outputs)?,
                Operator::LeftParen => break,
            };
            outputs.push(result);
            operators.pop();
        }
        Ok(())
    }

    fn reduce_precedence(outputs: &mut Vec<u64>, operators: &mut Vec<Operator>) -> Result<(), EvaluationError> {
        while operators.len() > 0 {
            let result = match operators[operators.len() - 1] {
                Operator::Add => Self::add(outputs)?,
                Operator::Mul => break,
                Operator::LeftParen => break,
            };
            outputs.push(result);
            operators.pop();
        }
        Ok(())
    }

    fn evaluate(&self) -> Result<u64, EvaluationError> {
        let mut outputs = Vec::new();
        let mut operators = Vec::new();

        for t in self.tokens.iter() {
            match t {
                Token::Int(i) => outputs.push(*i),
                Token::Plus => {
                    Self::reduce(&mut outputs, &mut operators)?;
                    operators.push(Operator::Add);
                },
                Token::Asterisk => {
                    Self::reduce(&mut outputs, &mut operators)?;
                    operators.push(Operator::Mul);
                },
                Token::LeftParen => operators.push(Operator::LeftParen),
                Token::RightParen => {
                    Self::reduce(&mut outputs, &mut operators)?;
                    operators.pop().ok_or(EvaluationError::MismatchedParens)?;
                }
            }
        }

        Self::reduce(&mut outputs, &mut operators)?;

        if outputs.len() != 1 {
            Err(EvaluationError::InvalidExpression)
        } else {
            Ok(outputs.pop().unwrap())
        }
    }

    fn evaluate_precedence(&self) -> Result<u64, EvaluationError> {
        let mut outputs = Vec::new();
        let mut operators = Vec::new();

        for t in self.tokens.iter() {
            match t {
                Token::Int(i) => outputs.push(*i),
                Token::Plus => {
                    Self::reduce_precedence(&mut outputs, &mut operators)?;
                    operators.push(Operator::Add);
                },
                Token::Asterisk => {
                    Self::reduce(&mut outputs, &mut operators)?;
                    operators.push(Operator::Mul);
                },
                Token::LeftParen => operators.push(Operator::LeftParen),
                Token::RightParen => {
                    Self::reduce(&mut outputs, &mut operators)?;
                    operators.pop().ok_or(EvaluationError::MismatchedParens)?;
                }
            }
        }

        Self::reduce(&mut outputs, &mut operators)?;

        if outputs.len() != 1 {
            Err(EvaluationError::InvalidExpression)
        } else {
            Ok(outputs.pop().unwrap())
        }
    }
}

impl FromStr for Expression {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.chars().filter_map(|c| match c {
            ' ' => None,
            '0'..='9' => Some(Ok(Token::Int(c as u64 - '0' as u64))),
            '+' => Some(Ok(Token::Plus)),
            '*' => Some(Ok(Token::Asterisk)),
            '(' => Some(Ok(Token::LeftParen)),
            ')' => Some(Ok(Token::RightParen)),
            c => Some(Err(ParseError::InvalidChar(c))),
        }).collect::<Result<_, _>>()?;
        Ok(Self {
            tokens,
        })
    }
}

struct Day18;
impl Problem for Day18 {
    type Input = Vec<Expression>;
    type Part1Output = u64;
    type Part2Output = u64;
    type Error = EvaluationError;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut result = 0;
        for expr in input.iter() {
            result += expr.evaluate()?;
        }
        Ok(result)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut result = 0;
        for expr in input.iter() {
            result += expr.evaluate_precedence()?;
        }
        Ok(result)
    }
}

fn main() {
    solve::<Day18>("input").unwrap();
}
