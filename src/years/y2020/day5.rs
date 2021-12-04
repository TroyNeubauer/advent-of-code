use std::collections::{HashMap, HashSet};

use crate::traits::*;

pub struct S;

fn decode_id(line: &str) -> i32 {
    let mut number = 0;
    for c in line.chars() {
        number *= 2;
        if c == 'B' || c == 'R' {
            number += 1;
        }
    }
    number
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        input
            .lines()
            .map(|line| decode_id(line))
            .max()
            .unwrap()
            .into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut ids = HashSet::new();
        for line in input.lines() {
            let id = decode_id(line);
            ids.insert(id);
        }
        let max_id = *ids.iter().max().unwrap();
        let min_id = *ids.iter().min().unwrap();
        for i in min_id..max_id {
            if !ids.contains(&i) {
                return i.into();
            }
        }
        panic!()
    }
}
