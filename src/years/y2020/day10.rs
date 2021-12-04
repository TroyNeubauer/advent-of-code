use crate::traits::*;

pub struct S;

fn count_helper(lines: &[u32], mut index: usize) -> usize {
    if index == lines.len() - 1 {
        return 1;
    }
    let mut count = 0;
    let base = lines[index];
    index += 1;
    while index < lines.len() {
        let now = lines[index];
        let diff = now - base;
        //println!("comp {} and {}. d={}", base, now, diff);
        if diff > 3 {
            //println!("diff between {} and {} is too large!", base, now);
            break;
        }
        count += count_helper(lines, index);

        index += 1;
    }
    count
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut nums = input.nums();
        nums.push(nums.iter().max().unwrap() + 3);
        nums.push(0);
        nums.sort();
        let mut map: HashMap<u32, u32> = HashMap::new();
        for i in 1..nums.len() {
            let now = nums[i];
            let last = nums[i - 1];
            let diff = now - last;
            println!("diff {}", diff);
            match map.get(&diff).cloned() {
                Some(value) => map.insert(diff, value + 1),
                None => map.insert(diff, 1),
            };
        }
        (map.get(&1).unwrap() * map.get(&3).unwrap()).into()
    }

    fn part2(&self, input: Input) -> Output {
        println!("WARN: Part 2 never fully implemented");
        let mut nums = input.nums();
        nums.push(0);
        nums.sort();
        nums.push(nums[nums.len() - 1] + 3);
        let mut count = 1;
        let mut start = 0;
        let mut end = 1;
        loop {
            if end == nums.len() || nums[end] - nums[end - 1] == 3 {
                count *= count_helper(&nums[start..end], 0);
                start = end;
                if end == nums.len() {
                    break;
                }
            }
            end += 1;
        }

        count.into()
    }
}
