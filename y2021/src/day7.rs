use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let positions: Vec<i32> = input.nums_comma_separated();
        let average: i32 = positions.iter().sum::<i32>() / positions.len() as i32;
        let min: i32 = *positions.iter().min().unwrap();
        let max: i32 = *positions.iter().max().unwrap();
        let mut i = 0;
        let cost = |positions: &Vec<i32>, dest: i32| {
            let mut c: i32 = 0;
            for p in positions {
                c += (dest - *p).abs();
            }

            c
        };
        let mut best_cost = 999999999;
        loop {
            let pos_abs = average + i;
            let neg_abs = average - i;
            if pos_abs > max || neg_abs < min {
                break;
            }
            let pos = cost(&positions, pos_abs);
            let neg = cost(&positions, neg_abs);
            if pos < best_cost {
                best_cost = pos;
            }

            if neg < best_cost {
                best_cost = neg;
            }

            i += 1;
        }
        best_cost.into()
    }

    fn part2(&self, input: Input) -> Output {
        let positions: Vec<i32> = input.nums_comma_separated();
        let average: i32 = positions.iter().sum::<i32>() / positions.len() as i32;
        let min: i32 = *positions.iter().min().unwrap();
        let max: i32 = *positions.iter().max().unwrap();
        let mut i = 0;
        let cost = |positions: &Vec<i32>, dest: i32| {
            let mut c: i32 = 0;
            for p in positions {
                let distance = (dest - *p).abs();
                for i in 1..=distance {
                    c += i;
                }
            }

            c
        };
        let mut best_cost = 999999999;
        loop {
            let pos_abs = average + i;
            let neg_abs = average - i;
            if pos_abs > max || neg_abs < min {
                break;
            }
            let pos = cost(&positions, pos_abs);
            let neg = cost(&positions, neg_abs);
            if pos < best_cost {
                best_cost = pos;
            }

            if neg < best_cost {
                best_cost = neg;
            }

            i += 1;
        }
        best_cost.into()
    }
}
