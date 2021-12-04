use std::collections::{HashMap, HashSet};

use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut counter = 0;
        for group in input.as_str().split("\n\n") {
            let mut set: HashSet<char> = HashSet::new();
            for line in group.lines() {
                for c in line.chars() {
                    set.insert(c);
                }
            }

            counter += set.len();
        }
        counter.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut counter = 0;
        for group in input.as_str().split("\n\n") {
            let mut set: HashMap<char, u8> = HashMap::new();
            let mut people = 0;
            for line in group.lines() {
                for c in line.chars() {
                    match set.get(&c).cloned() {
                        Some(count) => set.insert(c, count.clone() + 1),
                        None => set.insert(c, 1),
                    };
                }
                people += 1;
            }
            for (_c, count) in &set {
                if *count as usize == people {
                    counter += 1;
                }
            }
        }
        counter.into()
    }
}
