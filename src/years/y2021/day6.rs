use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut fish: Vec<u32> = input.nums_comma_separated();

        for _ in 0..80 {
            let mut to_add = Vec::new();
            for f in fish.iter_mut() {
                if *f == 0 {
                    *f = 6;
                    to_add.push(8);
                } else {
                    *f -= 1;
                }
            }
            fish.extend(to_add);
        }

        fish.len().into()
    }

    fn part2(&self, input: Input) -> Output {
        let fish: Vec<u32> = input.nums_comma_separated(); 

        let mut days_until_born = [0usize; 9];
        for f in fish {
            days_until_born[f as usize] += 1;
        }
        for _ in 0..256 {
            let born_count = days_until_born[0];
            for i in 1..days_until_born.len() {
                days_until_born[i - 1] = days_until_born[i];
            }
            days_until_born[8] = born_count;
            days_until_born[6] += born_count;
        }

        days_until_born.iter().sum::<usize>().into()
    }
}
