use problem::{Problem, solve};

struct Day10;
impl Problem for Day10 {
    type Input = Vec<i32>;
    type Part1Output = u32;
    type Part2Output = u64;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut sorted = input.clone();
        sorted.push(0);
        sorted.as_mut_slice().sort();
        sorted.push(sorted[sorted.len() - 1] + 3);

        let mut count_1 = 0;
        let mut count_3 = 0;
        for i in 1..sorted.len() {
            match sorted[i] - sorted[i - 1] {
                1 => count_1 += 1,
                3 => count_3 += 1,
                _ => (),
            }
        }

        Ok(count_1 * count_3)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut sorted = input.clone();
        sorted.push(0);
        sorted.as_mut_slice().sort();
        sorted.push(sorted[sorted.len() - 1] + 3);

        let mut counts = vec![0; sorted.len()];
        counts[0] = 1;

        for i in 1..sorted.len() {
            for j in 0..i {
                if (sorted[i] - sorted[j]).abs() <= 3 {
                    counts[i] += counts[j];
                }
            }
        }

        Ok(counts[sorted.len() - 1])
    }
}

fn main() {
    solve::<Day10>("input").unwrap();
}
