use util::{runner_main, AocDay, Input, Output};

struct Day1;

/// This function takes input for day 1 and returns an iterator of the sum of elf calories
fn elves(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
}

impl AocDay for Day1 {
    fn part1(&self, i: Input) -> Output {
        elves(i.as_str()).max().unwrap().into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut elves: Vec<_> = elves(i.as_str()).collect();
        elves.sort();
        elves[elves.len() - 3..].iter().sum::<i32>().into()
    }
}

fn main() {
    let d = Day1;
    runner_main(&d, 2022, 1);
}
