use std::collections::HashSet;

use termcolor::Color;
use util::{
    runner_main, AocDay, Direction, HighlightSpec, Input, IntoEnumeratedCells, Matrix, Output,
};

struct Day8;

// 30373
// 25512
// 65332
// 33549
// 35390

impl AocDay for Day8 {
    fn part1(&self, i: Input) -> Output {
        let mut trees: HashSet<(usize, usize)> = HashSet::new();
        let mat = Matrix::new_from_chars(i.as_str()).unwrap();
        for (row_start, col_start, _) in mat.iter().enumerate_cells() {
            if mat.is_edge(row_start, col_start) {
                /*mat.print_as_chars_and_highlight(HighlightSpec {
                    primary_row: row_start,
                    primary_col: col_start,
                    primary_color: Some(Color::Magenta),
                    secondary_cells: trees.iter().copied(),
                    secondary_color: Some(Color::Green),
                });*/
                trees.insert((row_start, col_start));
                let mut last_height = *mat.get(row_start, col_start);
                let towards_center = mat.direction_to_center(row_start, col_start);
                for (row, col, &height) in mat
                    .traverse(row_start, col_start, towards_center)
                    .enumerate_cells()
                {
                    if height > last_height {
                        /*mat.print_as_chars_and_highlight(HighlightSpec {
                            primary_row: row,
                            primary_col: col,
                            primary_color: Some(Color::Rgb(100, 255, 100)),
                            secondary_cells: trees.iter().cloned(),
                            secondary_color: Some(Color::Green),
                        });*/
                        trees.insert((row, col));
                        last_height = height;
                    } else {
                        // too short
                        /*mat.print_as_chars_and_highlight(HighlightSpec {
                            primary_row: col,
                            primary_col: row,
                            primary_color: Some(Color::Red),
                            secondary_cells: trees.iter().cloned(),
                            secondary_color: Some(Color::Green),
                        });*/
                    }
                }
            }
        }

        mat.print_as_chars_and_highlight(HighlightSpec {
            primary_row: 0,
            primary_col: 0,
            primary_color: None,
            secondary_cells: trees.iter().cloned(),
            secondary_color: Some(Color::Green),
        });

        trees.len().into()
    }

    fn part2(&self, i: Input) -> Output {
        let mat = Matrix::new_from_chars(i.as_str()).unwrap();
        mat.iter()
            .enumerate_cells()
            .map(
                |(row_start, col_start, &base_height): (usize, usize, &u8)| {
                    let score = Direction::all()
                        .iter()
                        .map(|&direction| {
                            let mut max_neighbor = None;
                            let mut trees_seen = 0;
                            mat.traverse(row_start, col_start, direction)
                                .enumerate_cells()
                                .take_while(|(row, col, &current_height)| {
                                    let p = current_height >= max_neighbor.unwrap_or(0)
                                        && current_height < base_height;
                                    max_neighbor = Some(current_height);
                                    trees_seen += 1;

                                    p
                                })
                                .for_each(|_| {});

                            trees_seen
                        })
                        .product::<usize>();
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
