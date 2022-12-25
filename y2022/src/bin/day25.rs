use std::str::FromStr;

use util::{runner_main, AocDay, Input, Output};

struct Day25;

struct Snafu(u64);

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: i64 = 0;
        let mut place = 1;
        for c in s.bytes().rev() {
            let digit: i64 = match c {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'=' => -2,
                b'-' => -1,
                _ => return Err(()),
            };
            result += digit * place;

            place *= 5;
        }
        Ok(Snafu(result as u64))
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num = self.0;
        let mut s = vec![];
        if num == 0 {
            s.push("0");
        }
        while num != 0 {
            let digit = num % 5;
            let c = match digit {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => {
                    // really minus two so we have to increase num
                    num += 5;
                    "="
                }
                4 => {
                    num += 5;
                    "-"
                }
                _ => unreachable!(),
            };
            s.push(c);

            num /= 5;
        }

        s.into_iter().rev().collect::<String>().fmt(f)
    }
}

impl AocDay for Day25 {
    fn part1(&self, i: Input) -> Output {
        let sum: u64 = i
            .lines()
            .map(|l| l.parse::<Snafu>().unwrap())
            .map(|s| s.0)
            .sum();

        format!("{}", Snafu(sum)).into()
    }

    fn part2(&self, i: Input) -> Output {
        i.into_inner().into()
    }
}

fn main() {
    let d = Day25;
    runner_main(&d, 2022, 25);
}
