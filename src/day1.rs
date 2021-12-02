
use crate::traits::AocDay;

struct Day1;

impl AocDay for Day1 {
    fn part1(input: crate::traits::Input) -> Result<crate::traits::Output, crate::traits::Error> {
        let input: Vec<u32> = input.nums();

        let mut count = 0;
        for i in 0..input.len() - 1 {
            if input[i + 1] > input[i] {
                count += 1;
            }
        }

        Ok(count.into())
    }

    fn part2(input: crate::traits::Input) -> Result<crate::traits::Output, crate::traits::Error> {
        let input = input.nums();
        let mut count = 0;
        for i in 0..input.len() {
            let first = sum(&input, i);
            let second = sum(&input, i + 1);
            if second > first {
                count += 1;
            }
        }
        Ok(count.into())
    }
}

fn sum(data: &[u32], index: usize) -> u32 {
    data.iter().skip(index).take(3).sum()
}

