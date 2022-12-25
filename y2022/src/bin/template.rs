use util::{runner_main, AocDay, Input, Output};

struct Day9;

impl AocDay for Day9 {
    fn part1(&self, i: Input) -> Output {
        i.into_inner().into()
    }

    fn part2(&self, i: Input) -> Output {
        i.into_inner().into()
    }
}

fn main() {
    let d = Day9;
    runner_main(&d, 2022, 9);
}
