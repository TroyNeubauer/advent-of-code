use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let input: Matrix<u8> = Matrix::new_from_chars(input.as_str()).unwrap();
        let cols = input.cols();
        let mut last_mat = Shifter::<Matrix<_>, 1>::new(input);
        loop {
            let it = {
                let mat = last_mat.get_current().unwrap();

                mat.enumerated_iter().map(|(row, col, s)| {
                    let around_count: usize = mat
                        .neighbor_iter(row, col)
                        .map(|s| if *s == b'#' { 1 } else { 0 })
                        .sum();
                    if *s == b'L' && around_count == 0 {
                        b'#'
                    } else if *s == b'#' && around_count >= 4 {
                        b'L'
                    } else {
                        *s
                    }
                })
            };

            let new_seats: Matrix<u8> = Matrix::new_from_iterator(cols, it);
            let last = last_mat.shift(new_seats);

            if let Some(last) = last {
                let current = last_mat.get_current().unwrap();
                if &last == current {
                    return current
                        .iter()
                        .map(|s| if *s == b'#' { 1 } else { 0 })
                        .sum::<usize>()
                        .into();
                }
            }
        }
    }

    fn part2(&self, input: Input) -> Output {
        let input: Matrix<u8> = Matrix::new_from_chars(input.as_str()).unwrap();
        //input.print_with(|s| *s as char);
        let cols = input.cols();
        let mut last_mat = Shifter::<Matrix<_>, 1>::new(input);
        let mut i = 0;
        loop {
            let it = {
                let mat = last_mat.get_current().unwrap();

                mat.enumerated_iter().map(|(row, col, s)| {
                    if *s == b'.' {
                        return *s;
                    }
                    let around_count: usize = mat
                        .enumerated_neighbor_iter(row, col)
                        .map(|(n_row, n_col, _s)| {
                            let r_dir = row as isize - n_row as isize;
                            let c_dir = col as isize - n_col as isize;
                            let mut p_row = n_row as isize;
                            let mut p_col = n_col as isize;
                            loop {
                                if p_row < 0 || p_col < 0 {
                                    break 0;
                                }
                                match mat.try_get(p_row as usize, p_col as usize) {
                                    Some(s) => {
                                        if *s == b'#' {
                                            //We see a taken seat
                                            break 1;
                                        }
                                        if *s == b'L' {
                                            //We see an empty seat
                                            break 0;
                                        }
                                    }
                                    None => break 0,
                                }
                                p_row += r_dir;
                                p_col += c_dir;
                            }
                        })
                        .sum();

                    if *s == b'L' && around_count == 0 {
                        b'#'
                    } else if *s == b'#' && around_count >= 5 {
                        b'L'
                    } else {
                        *s
                    }
                })
            };

            let new_seats: Matrix<u8> = Matrix::new_from_iterator(cols, it);
            //new_seats.print_with(|s| *s as char);
            let last = last_mat.shift(new_seats);

            i += 1;
            if let Some(last) = last {
                let current = last_mat.get_current().unwrap();
                if &last == current || i == 2 {
                    return current
                        .iter()
                        .map(|s| if *s == b'#' { 1 } else { 0 })
                        .sum::<usize>()
                        .into();
                }
            }
        }
    }
}
