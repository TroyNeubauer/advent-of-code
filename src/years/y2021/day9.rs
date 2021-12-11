use crate::traits::*;

pub struct S;

fn explore(row: usize, col: usize, mat: &Matrix<u8>, visited: &mut Matrix<bool>) -> usize {
    if *visited.get(row, col) {
        return 0;
    }
    visited.set(row, col, true);
    if *mat.get(row, col) >= 9 {
        return 0;
    }
    let mut size = 1;
    for (row, col, value) in mat.strict_enum_neighbor_iter(row, col) {
        size += explore(row, col, mat, visited);
    }
    size
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mat: Matrix<u8> = Matrix::new_from_chars(input.as_str()).unwrap();
        let mat = mat.map(|c| c - b'0');
        let mut count: usize = 0;
        for (row, col, center) in mat.enumerated_iter() {
            let mut low = true;
            for n in mat.strict_neighbor_iter(row, col) {
                if center >= n {
                    low = false;
                    break;
                }
            }
            if low {
                println!("{} is low", center);
                count += *center as usize + 1;
            }
        }
        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mat: Matrix<u8> = Matrix::new_from_chars(input.as_str()).unwrap();
        let mut visited = Matrix::new_with_value(mat.rows(), mat.cols(), false);
        let mut basins = Vec::new();
        let mat = mat.map(|c| c - b'0');
        for (row, col, center) in mat.enumerated_iter() {
            let mut low = true;
            for n in mat.strict_neighbor_iter(row, col) {
                if center >= n {
                    low = false;
                    break;
                }
            }
            if low {
                println!("exploring basin {} at {}, {}", center, row, col);
                let size = explore(row, col, &mat, &mut visited);
                println!("Got size: {}", size);
                basins.push(size);
            }
        }
        basins.sort();
        basins.reverse();
        let mut a = 1;
        for val in basins[0..3].iter() {
            a *= val
        }
        a.into()
    }
}
