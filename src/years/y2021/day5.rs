use itertools::Itertools;

use crate::traits::*;

pub struct S;

fn parse(lines: Vec<&str>) -> Vec<((i32, i32), (i32, i32))> {
    lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split(",")
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let lines = parse(input.lines().collect());
        let size = (lines
            .iter()
            .map(|(a, b)| a.0.max(b.0).max(a.1).max(b.1))
            .max()
            .unwrap()
            + 1) as usize;

        let mut mat: Matrix<i32> = Matrix::square(size, 0);
        for (a, b) in lines {
            let x_dir = (b.0 - a.0).clamp(-1, 1);
            let y_dir = (b.1 - a.1).clamp(-1, 1);
            if !(a.0 == b.0 || a.1 == b.1) {
                continue;
            }
            let mut p = a;
            loop {
                let c = *mat.get(p.0 as usize, p.1 as usize);
                mat.set(p.0 as usize, p.1 as usize, c + 1);
                if p == b {
                    break;
                }
                p.0 += x_dir;
                p.1 += y_dir;
            }
        }
        let mut count = 0;
        for square in mat.iter() {
            if *square >= 2 {
                count += 1;
            }
        }

        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        let lines = parse(input.lines().collect());
        let size = (lines
            .iter()
            .map(|(a, b)| a.0.max(b.0).max(a.1).max(b.1))
            .max()
            .unwrap()
            + 1) as usize;

        let mut mat: Matrix<i32> = Matrix::square(size, 0);
        for (a, b) in lines {
            let x_dir = (b.0 - a.0).clamp(-1, 1);
            let y_dir = (b.1 - a.1).clamp(-1, 1);
            let mut p = a;
            loop {
                let c = *mat.get(p.0 as usize, p.1 as usize);
                mat.set(p.0 as usize, p.1 as usize, c + 1);
                if p == b {
                    break;
                }
                p.0 += x_dir;
                p.1 += y_dir;
            }
        }
        let mut count = 0;
        for square in mat.iter() {
            if *square >= 2 {
                count += 1;
            }
        }

        count.into()
    }
}
