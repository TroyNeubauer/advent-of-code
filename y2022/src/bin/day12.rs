use util::{runner_main, AocDay, Input, Output};

struct Day12;

impl AocDay for Day12 {
    fn part1(&self, i: Input) -> Output {
        i.into_inner().into()
    }

    fn part2(&self, i: Input) -> Output {
        i.into_inner().into()
    }
}

fn main() {
    let d = Day12;
    runner_main(&d, 2022, 12);
}
