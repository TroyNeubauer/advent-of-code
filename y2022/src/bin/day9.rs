use std::collections::HashSet;

use util::{runner_main, AocDay, Input, Output};

struct Day9;

impl AocDay for Day9 {
    fn part1(&self, i: Input) -> Output {
        let mut visited = HashSet::new();
        let mut head = (0isize, 0isize);
        let mut tail = (0isize, 0isize);
        let in_same_row_col =
            |head: (isize, isize), tail: (isize, isize)| head.0 == tail.0 || head.1 == tail.1;
        visited.insert(tail);
        for line in i.lines() {
            let mut dir = String::new();
            let mut count = 0u8;
            scanf::sscanf!(line, "{} {}", dir, count).unwrap();
            for _ in 0..count {
                match dir.as_bytes()[0] {
                    b'R' => {
                        head.1 += 1;
                    }
                    b'L' => {
                        head.1 -= 1;
                    }
                    b'U' => {
                        head.0 -= 1;
                    }
                    b'D' => {
                        head.0 += 1;
                    }
                    _ => panic!(),
                }
                if in_same_row_col(head, tail) {
                    let delta = (head.0 - tail.0, head.1 - tail.1);
                    dbg!(delta);
                }
            }
        }
        visited.len().into()
    }

    fn part2(&self, i: Input) -> Output {
        i.into_inner().into()
    }
}

fn main() {
    let d = Day9;
    runner_main(&d, 2022, 9);
}
