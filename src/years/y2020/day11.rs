use crate::traits::*;
use std::str::FromStr;

pub struct S;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Position::Empty),
            "#" => Ok(Position::Occupied),
            "." => Ok(Position::Floor),
            _ => Err(()),
        }
    }
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut input: Matrix<Position> = Matrix::new(input.as_str()).unwrap();
        let mut last_mat = None;
        let cols = input.cols();
        loop {
            let it = input.enumerated_iter().map(|(row, col, s)| *s);
            let new_seats: Matrix<Position> = Matrix::new_from_iterator(cols, it);

            if let Some(last) = &last_mat {
                if last == &input {
                    return input
                        .iter()
                        .map(|s| if *s == Position::Occupied { 1 } else { 0 })
                        .sum::<usize>()
                        .into();
                }
            }
        }
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
