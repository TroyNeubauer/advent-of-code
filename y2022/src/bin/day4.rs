#![feature(iter_array_chunks)]

use std::ops::RangeInclusive;

use util::{runner_main, AocDay, Input, Output};

struct Day4;

impl AocDay for Day4 {
    fn part1(&self, i: Input) -> Output {
        i.lines()
            .map(|line| {
                let mut a1 = 0;
                let mut a2 = 0;
                let mut b1 = 0;
                let mut b2 = 0;
                scanf::sscanf!(line, "{}-{},{}-{}", a1, a2, b1, b2).unwrap();
                let a: RangeInclusive<usize> = a1..=a2;
                let b: RangeInclusive<usize> = b1..=b2;

                if a.contains(&b.start()) && a.contains(&b.end())
                    || b.contains(&a.start()) && b.contains(&a.end())
                {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>()
            .into()
    }

    fn part2(&self, i: Input) -> Output {
        i.lines()
            .map(|line| {
                let mut a1 = 0;
                let mut a2 = 0;
                let mut b1 = 0;
                let mut b2 = 0;
                scanf::sscanf!(line, "{}-{},{}-{}", a1, a2, b1, b2).unwrap();
                let a: RangeInclusive<usize> = a1..=a2;
                let b: RangeInclusive<usize> = b1..=b2;

                if a.contains(&b.start()) || a.contains(&b.end()) {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>()
            .into()
    }
}

fn main() {
    let d = Day4;
    runner_main(&d, 2022, 4);
}
