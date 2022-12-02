use crate::traits::*;

pub struct S;

impl AocDay for S {
    fn part1(&self, input: crate::traits::Input) -> Output {
        let mat = Matrix::new_from_single_nums(input.as_str()).unwrap();
        let end = ((mat.cols() - 1), (mat.rows() - 1)); 

        let (_path, cost) = mat.pathfind((0, 0), end).unwrap();
        cost.into()
    }

    fn part2(&self, input: crate::traits::Input) -> Output {
        let small = Matrix::new_from_single_nums(input.as_str()).unwrap();
        let mut mat: Matrix<u8> = Matrix::new_with_value(small.rows() * 5, small.cols() * 5, 0);
        for row in 0..mat.rows() {
            for col in 0..mat.cols() {
                let big_row = row / small.rows();
                let big_col = col / small.cols();

                let o_val = small.get(row % small.rows(), col % small.cols());
                let mut val = big_col as u8 + big_row as u8 + *o_val;
                if val >= 10 {
                    val = val % 10 + 1;
                }

                mat.set(row, col, val);
            }
        }

        let end = (mat.cols() - 1, mat.rows() - 1); 

        let (_path, cost) = mat.pathfind((0, 0), end).unwrap();
        cost.into()
    }
}
