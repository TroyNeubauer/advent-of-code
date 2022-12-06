use std::collections::BTreeSet;

use util::{runner_main, AocDay, Input, Output};

struct Day6;

impl AocDay for Day6 {
    fn part1(&self, i: Input) -> Output {
        let s = i.as_bytes();
        for i in 4..s.len() {
            let values = &s[i - 4..i];
            let set = BTreeSet::from_iter(values.into_iter());
            if set.len() == 4 {
                return i.into();
            }
        }
        panic!()
    }

    fn part2(&self, i: Input) -> Output {
        let s = i.as_bytes();
        for i in 14..s.len() {
            let values = &s[i - 14..i];
            let set = BTreeSet::from_iter(values.into_iter());
            if set.len() == 14 {
                return i.into();
            }
        }
        panic!()
    }
}

fn main() {
    let d = Day6;
    runner_main(&d, 2022, 6);
}
