use std::collections::HashMap;

use problem::{CSV, One, Problem, solve};



struct Day15;
impl Problem for Day15 {
    type Input = One<CSV<u32>>;
    type Part1Output = u32;
    type Part2Output = u32;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut time_last_spoken = HashMap::new();
        for (i, n) in input.0.values[0..input.0.values.len() - 1].iter().enumerate() {
            time_last_spoken.insert(*n, i as u32);
        }
        let mut last_number = *input.0.values.last().unwrap();
        for time in input.0.values.len()..2020 {
            let next = match time_last_spoken.get(&last_number) {
                Some(t) => (time - 1) as u32 - t,
                None => 0,
            };
            time_last_spoken.insert(last_number, (time - 1) as u32);
            last_number = next;
        }
        Ok(last_number)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut time_last_spoken = HashMap::new();
        for (i, n) in input.0.values[0..input.0.values.len() - 1].iter().enumerate() {
            time_last_spoken.insert(*n, i as u32);
        }
        let mut last_number = *input.0.values.last().unwrap();
        for time in input.0.values.len()..30000000 {
            let next = match time_last_spoken.get(&last_number) {
                Some(t) => (time - 1) as u32 - t,
                None => 0,
            };
            time_last_spoken.insert(last_number, (time - 1) as u32);
            last_number = next;
        }
        Ok(last_number)
    }
}

fn main() {
    solve::<Day15>("input").unwrap();
}
