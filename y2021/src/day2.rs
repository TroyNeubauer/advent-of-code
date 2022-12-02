use crate::traits::*;

pub struct S;

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {

        let mut horiz = 0;
        let mut depth = 0;

        for command in input.lines() {
            let mut parts = command.split(' ');
            let c = parts.next().unwrap();
            let num = parts.next().unwrap().parse::<u32>().unwrap();
            match c {
                "down" => depth += num,
                "up" => depth -= num,
                "forward" => {
                    horiz += num;
                }
                //"down" => depth += num,
                _ => panic!("{}", c),
            }
        }
        (horiz * depth).into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut horiz = 0;
        let mut aim = 0;
        let mut depth = 0;

        for command in input.lines() {
            let mut parts = command.split(' ');
            let c = parts.next().unwrap();
            let num = parts.next().unwrap().parse::<u32>().unwrap();
            match c {
                "down" => aim += num,
                "up" => aim -= num,
                "forward" => {
                    horiz += num;
                    depth += aim * num;
                }
                //"down" => depth += num,
                _ => panic!("{}", c),
            }
        }
        (horiz * depth).into()
    }
}
