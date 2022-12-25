use std::collections::HashSet;

use util::{runner_main, AocDay, Direction, Input, Matrix, Output, Point, SignedPoint};

struct Day9;

impl AocDay for Day9 {
    fn part1(&self, i: Input) -> Output {
        let mut visited = HashSet::new();
        let start = Point::new(1024, 1024);
        let mut head = start;
        let mut tail = start;
        visited.insert(tail);

        for line in i.lines() {
            let mut dir = String::new();
            let mut count = 0u8;

            scanf::sscanf!(line, "{} {}", dir, count).unwrap();
            let direction: Direction = dir.as_bytes()[0].try_into().unwrap();

            for _ in 0..count {
                head = head.offset(direction);
                let delta = head - tail;
                let dist = delta.manhattan_distance();
                if delta.is_axis_aligned() {
                    if dist >= 2 {
                        // tail follows head
                        let to_head = delta.try_to_direction().unwrap();
                        tail = tail.offset(to_head);
                    }
                } else {
                    if dist >= 3 {
                        let delta =
                            SignedPoint::new(delta.row.clamp(-1, 1), delta.col.clamp(-1, 1));
                        // 2 is fine for a diagional
                        tail += delta;
                    }
                }
                visited.insert(tail);
            }
        }
        visited.len().into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut visited = HashSet::new();
        const START: Point = Point::new(1024, 1024);
        let mut knots = [START; 10];
        let tail = |knots: &[Point; 10]| *knots.last().unwrap();
        visited.insert(tail(&knots));

        for line in i.lines() {
            let mut dir = String::new();
            let mut count = 0u8;

            scanf::sscanf!(line, "{} {}", dir, count).unwrap();
            let direction: Direction = dir.as_bytes()[0].try_into().unwrap();

            for _ in 0..count {
                knots[0] = knots[0].offset(direction);
                for i in 1..knots.len() {
                    let delta = knots[i - 1] - knots[i];
                    let dist = delta.manhattan_distance();
                    if delta.is_axis_aligned() {
                        if dist >= 2 {
                            // tail follows head
                            let to_head = delta.try_to_direction().unwrap();
                            knots[i] = knots[i].offset(to_head);
                        }
                    } else {
                        if dist >= 3 {
                            let delta =
                                SignedPoint::new(delta.row.clamp(-1, 1), delta.col.clamp(-1, 1));
                            // 2 is fine for a diagional
                            knots[i] += delta;
                        }
                    }
                }
                visited.insert(tail(&knots));
            }
        }
        visited.len().into()
    }
}

fn main() {
    let d = Day9;
    runner_main(&d, 2022, 9);
}
