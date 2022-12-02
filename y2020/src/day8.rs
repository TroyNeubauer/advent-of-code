use std::collections::HashMap;

use crate::traits::*;

pub struct S;

mod part1 {
    use super::*;

    pub fn execute(lines: Vec<&str>) -> i32 {
        let mut visit_count: HashMap<i32, u32> = HashMap::new();
        let mut rip: i32 = 0;
        let mut acc: i32 = 0;

        while rip >= 0 && (rip as usize) < lines.len() {
            if visit_count.get(&rip).is_some() {
                return acc;
            } else {
                visit_count.insert(rip, 1);
            }
            let line = lines[rip as usize];
            let split_index = line.chars().position(|c| c == ' ').unwrap();
            let inst = &line[..split_index];
            let arg: i32 = line[split_index + 1..].trim().parse().unwrap();
            match inst {
                "nop" => rip += 1,
                "acc" => {
                    rip += 1;
                    acc += arg
                }
                "jmp" => rip += arg,
                _ => panic!("bad instruction!"),
            }
        }

        acc
    }
}

mod part2 {
    use super::*;

    pub fn execute(lines: Vec<&str>) -> Option<i32> {
        for change in 0..lines.len() {
            let mut visit_count: HashMap<i32, u32> = HashMap::new();
            let mut rip: i32 = 0;
            let mut acc: i32 = 0;
            let mut abort = false;
            while rip >= 0 && (rip as usize) < lines.len() {
                if visit_count.get(&rip).is_some() {
                    abort = true;
                    break;
                } else {
                    visit_count.insert(rip, 1);
                }
                let line = lines[rip as usize];
                let split_index = line.chars().position(|c| c == ' ').unwrap();
                let mut inst = &line[..split_index];
                let arg: i32 = line[split_index + 1..].trim().parse().unwrap();
                if rip as usize == change {
                    match inst {
                        "nop" => inst = "jmp",
                        "jmp" => inst = "nop",
                        _ => (),
                    }
                }

                match inst {
                    "nop" => rip += 1,
                    "acc" => {
                        rip += 1;
                        acc += arg
                    }
                    "jmp" => rip += arg,
                    _ => panic!("bad instruction!"),
                }
            }
            if abort {
                continue;
            }

            return Some(acc);
        }

        None
    }
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        part1::execute(input.lines().collect()).into()
    }

    fn part2(&self, input: Input) -> Output {
        part2::execute(input.lines().collect()).unwrap().into()
    }
}
