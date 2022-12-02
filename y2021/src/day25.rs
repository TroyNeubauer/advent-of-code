use std::sync::Arc;

use crate::traits::*;

pub struct S;

fn step(mat: &Matrix<u8>) -> Matrix<u8> {
    let mut result = mat.clone();
    let rows = mat.rows();
    let cols = mat.cols();
    for (row, o_col, c) in mat.enumerated_iter() {
        let mut col = o_col;
        let c = *c;
        if c == b'>' {
            if *mat.get(row, (col + 1) % cols) == b'.' {
                col += 1;
                col %= cols;
            }
            result.set(row, o_col, b'.');
            result.set(row, col, c);
        }
    }

    let mat = result.clone();

    for (o_row, col, c) in mat.enumerated_iter() {
        let mut row = o_row;
        let c = *c;
        if c == b'v' {
            if *mat.get((row + 1) % rows, col) == b'.' {
                row += 1;
                row %= rows;
            }
            result.set(o_row, col, b'.');
            result.set(row, col, c);
        }
    }

    result
}

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut mat = Arc::new(Matrix::new_from_chars(input.as_str()).unwrap());
        let mut last = Arc::clone(&mat);
        let mut count = 0;
        loop {
            count += 1;

            last = Arc::clone(&mat);
            mat = Arc::new(step(&mat));
            if last == mat {
                break count.into();
            }
        }
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
