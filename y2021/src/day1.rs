use crate::traits::*;

pub struct S;

impl AocDay for S {
    fn part1(&self, input: crate::traits::Input) -> Output {
        let input: Vec<u32> = input.nums();

        let mut count = 0;
        for i in 0..input.len() - 1 {
            if input[i + 1] > input[i] {
                count += 1;
            }
        }

        count.into()
    }

    fn part2(&self, input: crate::traits::Input) -> Output {
        let input = input.nums();
        let mut count = 0;
        for i in 0..input.len() {
            let first = sum(&input, i);
            let second = sum(&input, i + 1);
            if second > first {
                count += 1;
            }
        }
        count.into()
    }
}

fn sum(data: &[u32], index: usize) -> u32 {
    data.iter().skip(index).take(3).sum()
}
