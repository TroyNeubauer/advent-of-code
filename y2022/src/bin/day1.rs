fn elves(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
}

pub fn part1(input: &str) -> i32 {
    elves(input).max().unwrap()
}

pub fn part2(input: &str) -> i32 {
    let mut elves: Vec<_> = elves(input).collect();
    elves.sort();
    elves[elves.len() - 3..].iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    println!("1 {}", part1(&input));
    println!("2 {}", part2(&input));
}
