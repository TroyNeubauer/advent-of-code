use crate::traits::*;

fn get_bad_number(lines: Vec<usize>, pre_length: usize, index: usize) -> Option<usize> {
    if lines.len() <= index as usize {
        return None;
    }
    let value = lines[index as usize];
    for i in 0..pre_length {
        for j in 0..pre_length {
            if i == j {
                continue;
            }
            let a = lines[(i + index - pre_length) as usize];
            let b = lines[(j + index - pre_length) as usize];
            if a + b == value {
                return get_bad_number(lines, pre_length, index + 1);
            }
        }
    }
    return Some(value);
}

fn get_range_sum(lines: &Vec<usize>, bad: usize) -> Option<usize> {
    return None;
}

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let num = if input.lines().collect::<Vec<_>>().len() > 50 {
            25
        } else {
            5
        };
        get_bad_number(input.nums(), num, num).unwrap().into()
    }

    fn part2(&self, input: Input) -> Output {
        let bad = self
            .part1(input.clone())
            .into_inner()
            .parse::<usize>()
            .unwrap();

        let nums = input.nums();
        for i in 0..nums.len() {
            let mut sum = 0;
            let mut j = i;
            while sum < bad {
                sum += nums[j];
                if sum == bad {
                    println!("found range {}..{}", i, j);
                    let min: usize = *nums[i..j].iter().min().unwrap();
                    let max: usize = *nums[i..j].iter().max().unwrap();
                    println!("min {}, max {}", min, max);
                    return (min + max).into();
                }
                j += 1;
            }
        }
        panic!()
    }
}
