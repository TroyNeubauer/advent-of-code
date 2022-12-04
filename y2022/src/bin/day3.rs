#![feature(iter_array_chunks)]

use util::{runner_main, AocDay, Input, Output};

struct Day3;

impl AocDay for Day3 {
    fn part1(&self, i: Input) -> Output {
        let mut total = 0;
        for line in i.lines() {
            let (a, b) = line.split_at(line.len() / 2);
            let mut common = None;
            for a in a.chars() {
                if b.contains(a) {
                    common = Some(a);
                    break;
                }
            }
            let common = common.unwrap();
            dbg!(common);
            if common.is_uppercase() {
                total += (common as u8 - b'A' + 27) as usize;
            } else {
                total += (common as u8 - b'a' + 1) as usize;
            }
        }
        total.into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut total = 0;
        for line in i.lines().array_chunks::<3>() {
            let mut common = None;
            for a in line[0].chars() {
                if line[1].contains(a) {
                    if line[2].contains(a) {
                        common = Some(a);
                        break;
                    }
                }
            }
            let common = common.unwrap();
            dbg!(common);
            if common.is_uppercase() {
                total += (common as u8 - b'A' + 27) as usize;
            } else {
                total += (common as u8 - b'a' + 1) as usize;
            }
        }
        total.into()
    }
}

fn main() {
    let d = Day3;
    runner_main(&d, 2022, 3);
}
