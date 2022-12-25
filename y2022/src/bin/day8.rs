use std::collections::HashSet;

use util::{runner_main, AocDay, Direction, Input, IntoEnumeratedCells, Matrix, Output};

struct Day8;

impl AocDay for Day8 {
    fn part1(&self, i: Input) -> Output {
        let mut trees: HashSet<(usize, usize)> = HashSet::new();
        let mat = Matrix::new_from_chars(i.as_str()).unwrap();
        for (row_start, col_start, _) in mat.iter().enumerate_cells() {
            if mat.is_edge(row_start, col_start) {
                trees.insert((row_start, col_start));
                let mut last_height = *mat.get(row_start, col_start);
                let towards_center = mat.direction_to_center(row_start, col_start);
                for (row, col, &height) in mat
                    .traverse(row_start, col_start, towards_center)
                    .enumerate_cells()
                {
                    if height > last_height {
                        trees.insert((row, col));
                        last_height = height;
                    } else {
                        // too short
                    }
                }
            }
        }

        trees.len().into()
    }

    fn part2(&self, i: Input) -> Output {
        let mat = Matrix::new_from_chars(i.as_str()).unwrap();
        mat.iter()
            .enumerate_cells()
            .map(
                |(row_start, col_start, &base_height): (usize, usize, &u8)| {
                    let mut score = 1;
                    for direction in Direction::all() {
                        let mut direction_score = 0;
                        for &current_height in mat.traverse(row_start, col_start, direction) {
                            direction_score += 1;

                            if base_height <= current_height {
                                break;
                            }
                        }

                        score *= direction_score;
                    }
                    score
                },
            )
            .max()
            .unwrap()
            .into()
    }
}

fn main() {
    let d = Day8;
    runner_main(&d, 2022, 8);
}
