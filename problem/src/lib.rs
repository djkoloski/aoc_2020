use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

pub trait Input: Sized {
    type Error: Debug;

    fn parse<R: BufRead>(reader: R) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub enum ParseLinesError<T> {
    IoError(io::Error),
    ParseLine {
        line_number: usize,
        error: T,
    },
}

impl<T> From<io::Error> for ParseLinesError<T> {
    fn from(e: io::Error) -> Self {
        ParseLinesError::IoError(e)
    }
}

impl<T: FromStr> Input for Vec<T>
where
    T::Err: Debug,
{
    type Error = ParseLinesError<T::Err>;

    fn parse<R: BufRead>(reader: R) -> Result<Self, Self::Error> {
        Ok(
            reader.lines()
                .enumerate()
                .map(|(line_number, line)| line?.parse().map_err(|error| ParseLinesError::ParseLine { line_number: line_number + 1, error }))
                .collect::<Result<Vec<_>, _>>()?
        )
    }
}

pub trait Problem {
    type Input: Input;
    type Part1Output: Display;
    type Part2Output: Display;
    type Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error>;
    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error>;
}

#[derive(Debug)]
pub enum SolveError<P, E> {
    IoError(io::Error),
    ParseInput(P),
    SolvePart1(E),
    SolvePart2(E),
}

impl<P, E> From<io::Error> for SolveError<P, E> {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

pub fn solve<P: Problem>(path: &str) -> Result<(P::Part1Output, P::Part2Output), SolveError<<P::Input as Input>::Error, P::Error>> {
    let input_file = BufReader::new(File::open(path)?);
    let input = P::Input::parse(input_file).map_err(|e| SolveError::ParseInput(e))?;

    let start = Instant::now();
    let part_1 = P::part_1(&input).map_err(|e| SolveError::SolvePart1(e))?;
    let duration = Instant::now().duration_since(start);

    println!("Part 1:\n  Solution: {}\n  Elapsed:  {} seconds", part_1, duration.as_secs_f64());

    let start = Instant::now();
    let part_2 = P::part_2(&input).map_err(|e| SolveError::SolvePart2(e))?;
    let duration = Instant::now().duration_since(start);

    println!("Part 2:\n  Solution: {}\n  Elapsed:  {} seconds", part_2, duration.as_secs_f64());

    Ok((part_1, part_2))
}
