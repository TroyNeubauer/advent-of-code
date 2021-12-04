use crate::traits::*;

pub struct S;

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut gamma_count = [0usize; 20];
        let bits = input.lines().next().unwrap().len();
        let lines: Vec<_> = input.lines().collect();

        for line in &lines {
            for (i, b) in line.bytes().enumerate().rev() {
                if b == b'1' {
                    gamma_count[i] += 1;
                }
            }
        }
        let mut gamma = 0;
        let mut elipison = 0;
        for bit in 0..bits {
            if gamma_count[bit] * 2 > lines.len() {
                gamma |= 1 << (bits - bit - 1);
            } else {
                elipison |= 1 << (bits - bit - 1);
            }
        }

        (gamma * elipison).into()
    }

    fn part2(&self, input: Input) -> Output {
        let bits = input.lines().next().unwrap().len();

        let values: Vec<u32> = input
            .lines()
            .filter_map(|l| u32::from_str_radix(l, 2).ok())
            .collect();

        let reduce = |mut values: Vec<u32>, perfer_high| {
            let mut result = None;
            for bit in (0..bits).rev() {
                let mut set_count = 0;
                for val in &values {
                    if (val >> bit) & 0b1 == 1 {
                        set_count += 1;
                    }
                }
                let to_keep = if perfer_high {
                    if set_count * 2 >= values.len() {
                        1
                    } else {
                        0
                    }
                } else {
                    if set_count * 2 >= values.len() {
                        0
                    } else {
                        1
                    }
                };
                values = values
                    .into_iter()
                    .filter(|v| (v >> bit) & 1 == to_keep)
                    .collect();

                if values.len() == 1 {
                    result = Some(values[0]);
                    break;
                }
            }
            result.unwrap()
        };

        let ox = reduce(values.clone(), true);
        let co2 = reduce(values.clone(), false);

        (ox * co2).into()
    }
}
