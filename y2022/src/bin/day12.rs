use util::{runner_main, AocDay, Input, IntoEnumeratedCells, Matrix, Output, Point};

struct Day12;

impl AocDay for Day12 {
    fn part1(&self, i: Input) -> Output {
        let mut mat = Matrix::new_from_chars(i).unwrap();

        let start = mat.find(|&v| v == b'S').unwrap();
        let end = mat.find(|&v| v == b'E').unwrap();
        mat.set(start.row, start.col, b'a');
        mat.set(end.row, end.col, b'z');

        let cost = |pos: &Point| (*pos - end).manhattan_distance();
        let successors = |pos: &Point| -> _ {
            let &current = mat.get(pos.row, pos.col);
            let it = mat
                .adjacent_neighbor_iter(pos.row, pos.col)
                .enumerate_cells()
                .collect::<Vec<_>>();

            it.into_iter()
                .filter(|(row, col, _)| *mat.get(*row, *col) <= current + 1)
                .map(|(row, col, _)| {
                    let point = Point::new(row, col);
                    (point, cost(&point))
                })
                .collect::<Vec<_>>()
        };

        let (steps, _cost) =
            pathfinding::prelude::astar(&start, |p| successors(p), cost, |&p| p == end).unwrap();
        (steps.len() - 1).into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut mat = Matrix::new_from_chars(i).unwrap();

        let start = mat.find(|&v| v == b'S').unwrap();
        let end = mat.find(|&v| v == b'E').unwrap();
        mat.set(start.row, start.col, b'a');
        mat.set(end.row, end.col, b'z');

        let starts: Vec<_> = mat
            .iter()
            .enumerate_cells()
            .filter(|(_, _, &c)| c == b'a')
            .collect();

        let min: usize = starts
            .into_iter()
            .map(|(start_row, start_col, _)| {
                let start = Point::new(start_row, start_col);
                let cost = |pos: &Point| (*pos - end).manhattan_distance();
                let successors = |pos: &Point| -> _ {
                    let &current = mat.get(pos.row, pos.col);
                    let it = mat
                        .adjacent_neighbor_iter(pos.row, pos.col)
                        .enumerate_cells()
                        .collect::<Vec<_>>();

                    it.into_iter()
                        .filter(|(row, col, _)| *mat.get(*row, *col) <= current + 1)
                        .map(|(row, col, _)| {
                            let point = Point::new(row, col);
                            (point, cost(&point))
                        })
                        .collect::<Vec<_>>()
                };

                pathfinding::prelude::astar(&start, |p| successors(p), cost, |&p| p == end)
                    .map(|(s, _c)| s.len() - 1)
            })
            .flatten()
            .min()
            .unwrap();
        min.into()
    }
}

fn main() {
    let d = Day12;
    runner_main(&d, 2022, 12);
}
