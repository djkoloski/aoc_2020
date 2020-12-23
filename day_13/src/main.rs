use std::{io, num};
use problem::{ProblemInput, Problem, solve};

struct Schedule {
    pub departure_time: u64,
    pub bus_schedule: Vec<Option<u64>>,
}

#[derive(Debug)]
enum ParseScheduleError {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    MissingDepartureTime,
    MissingBusSchedule,
}

impl From<io::Error> for ParseScheduleError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<num::ParseIntError> for ParseScheduleError {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl ProblemInput for Schedule {
    type Error = ParseScheduleError;

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();
        let departure_time = lines.next().ok_or(ParseScheduleError::MissingDepartureTime)??.parse()?;
        let bus_schedule = lines.next().ok_or(ParseScheduleError::MissingBusSchedule)??.split(',').map(|b| if b == "x" { Ok(None) } else { Ok(Some(b.parse()?)) }).collect::<Result<Vec<_>, num::ParseIntError>>()?;
        Ok(Schedule {
            departure_time,
            bus_schedule,
        })
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

struct Day13;
impl Problem for Day13 {
    type Input = Schedule;
    type Part1Output = u64;
    type Part2Output = u64;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let (time, bus) = input.bus_schedule.iter().filter_map(|b| *b).map(|b| (b - input.departure_time % b, b)).min().unwrap();
        Ok(time * bus)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut time = 0;
        let mut step = 1;
        for (i, b) in input.bus_schedule.iter().enumerate().filter_map(|(i, b)| b.map(|b| (i, b))) {
            while time % b != (b - i as u64 % b) % b {
                time += step;
            }
            step = lcm(step, b);
        }
        Ok(time)
    }
}

fn main() {
    solve::<Day13>("input").unwrap();
}
