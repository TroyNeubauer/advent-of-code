use util::{runner_main, AocDay, Input, Output};

struct Day1;

impl AocDay for Day1 {
    fn part1(&self, i: Input) -> Output {
        i.ints().map(|a| a / 3 - 2).sum::<i32>().into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut total = i
            .ints()
            .map(|a| {
                let mut old_mass = a;
                let mut total = 0;
                loop {
                    let mass = old_mass / 3 - 2;
                    if mass <= 0 {
                        break total;
                    }
                    old_mass = mass;
                    total += mass;
                }
            })
            .sum::<i32>();
        total.into()
    }
}

fn main() {
    let d = Day1;
    runner_main(&d, 2019, 1);
}
