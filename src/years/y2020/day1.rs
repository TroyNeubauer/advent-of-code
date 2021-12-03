use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let nums = input.nums();
        for i in &nums {
            let compliment: u32 = 2020 - i;
            if nums.contains(&compliment) {
                return (i * compliment).into();
            }
        }
        panic!();
    }

    fn part2(&self, input: Input) -> Output {
        let nums = input.nums();
        for i in &nums {
            for j in &nums {
                let compliment: i32 = 2020 - i - j;
                if nums.contains(&compliment) {
                    return (i * j * compliment).into();
                }
            }
        }
        panic!();
    }
}
