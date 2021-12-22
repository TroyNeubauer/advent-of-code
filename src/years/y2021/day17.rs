use itertools::Itertools;
use std::ops::RangeInclusive;

use crate::traits::*;

pub struct S;

fn simulate(
    mut dx: i32,
    mut dy: i32,
    target_x: &RangeInclusive<i32>,
    target_y: &RangeInclusive<i32>,
) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = y;
    loop {
        x += dx;
        y += dy;
        if y > max_y {
            max_y = y;
        }
        dx -= dx.signum();
        dy -= 1;
        match (target_x.contains(&x), target_y.contains(&y)) {
            (true, true) => return Some(max_y),
            (false, _) if dx == 0 => return None,
            (_, false) if dy < 0 && y < *target_y.start() => return None,
            _ => {}
        }
    }
}

fn parse(input: Input) -> Result<(i32, i32, i32, i32), scanfmt::ScanError> {
    let (x1, x2, y1, y2): (i32, i32, i32, i32);
    scanfmt::scanfmt!(
        input.as_str().trim(),
        "target area: x={}..{}, y=-{}..{}",
        x1,
        x2,
        y1,
        y2
    );

    Ok((x1, x2, y1, y2))
}

impl AocDay for S {
    fn part1(&self, input: crate::traits::Input) -> Output {
        let (x1, x2, y1, y2) = parse(input).unwrap();
        let x_target = x1..=x2;
        let y_target = y1..=y2;

        let x_range = i32::max(x1.abs(), x2.abs());
        let y_range = i32::max(y1.abs(), y2.abs());
        let x_range = (-x_range)..=x_range;
        let y_range = (-y_range)..=y_range;

        let x_target = &x_target;
        let y_target = &y_target;

        let maxys = x_range.into_iter()
            .cartesian_product(y_range.into_iter())
            .filter_map(move |(x, y)| simulate(x, y, x_target, y_target))
            .collect::<Vec<_>>();

        maxys.iter().max().unwrap().into()
    }

    fn part2(&self, input: crate::traits::Input) -> Output {
        todo!()
    }
}
