use std::collections::HashSet;

use crate::traits::*;

pub struct S;

struct Fold {
    axis: u8,
    coord: u32,
}

type Points = HashSet<(u32, u32)>;

struct Data {
    points: Points,
    folds: Vec<Fold>,
}

impl Fold {
    fn execute(self, points: &Points) -> Points {
        let mut result = HashSet::new();
        if self.axis == b'y' {
            for (x, y) in points {
                let new_y = if *y > self.coord {
                    self.coord - (*y - self.coord)
                } else {
                    *y
                };
                result.insert((*x, new_y));
            }
        } else {
            for (x, y) in points {
                let new_x = if *x > self.coord {
                    self.coord - (*x - self.coord)
                } else {
                    *x
                };
                result.insert((new_x, *y));
            }
        }

        result
    }
}

fn parse(input: String) -> Result<Data, ()> {
    let mut parts = input.split("\n\n");
    let points = parts.next().unwrap();
    let folds = parts.next().unwrap();
    let mut r_points = HashSet::new();
    let mut r_folds = Vec::new();
    for line in points.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        r_points.insert((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()));
    }
    for line in folds.lines() {
        let a = line.split(' ').nth(2).unwrap();
        let mut parts = a.split('=');
        let axis = parts.next().unwrap();
        let coord = parts.next().unwrap().parse::<u32>().unwrap();
        r_folds.push(Fold {
            axis: axis.as_bytes()[0],
            coord,
        });
    }
    Ok(Data {
        folds: r_folds,
        points: r_points,
    })
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut data = parse(input.into_inner()).unwrap();
        let fold = data.folds.remove(0);
        let out = fold.execute(&data.points);
        out.len().into()
    }

    fn part2(&self, input: Input) -> Output {
        let data = parse(input.into_inner()).unwrap();
        let mut points = data.points;
        for fold in data.folds {
            points = fold.execute(&points);
        }
        let line_width = points.iter().map(|(x, _y)| *x).max().unwrap() + 1;
        let lines = points.iter().map(|(_x, y)| *y).max().unwrap() + 1;
        for y in 0..lines {
            for x in 0..line_width {
                if points.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        let mut input = String::new();
        println!("Input the characters that you see?");
        std::io::stdin().read_line(&mut input).unwrap();
        input.into()
    }
}
