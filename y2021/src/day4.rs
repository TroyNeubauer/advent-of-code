use crate::traits::*;
use log::*;

pub struct S;

fn is_board_won(sorted_calls: &Vec<u32>, board: &Vec<Vec<u32>>) -> bool {
    let size = board.len() as i32;
    let check = |row_start: i32, col_start: i32, row_inc: i32, col_inc: i32| {
        for i in 0..size {
            let row = (row_start + i * row_inc) as usize;
            let col = (col_start + i * col_inc) as usize;
            let num = board[row][col];
            if sorted_calls.binary_search(&num).is_err() {
                return false;
            }
        }
        return true;
    };
    for row in 0..size {
        if check(row, 0, 0, 1) {
            return true;
        }
    }

    for col in 0..size {
        if check(0, col, 1, 0) {
            return true;
        }
    }

    /*
    Lol why did i think we needed diagonals
    if check(0, 0, 1, 1) {
        info!("Won major");
        return true;
    }

    if check(size - 1, 0, -1, 1) {
        info!("Won minor");
        return true;
    }
    */
    return false;
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut split = input.as_str().split("\n\n");
        let calls: Vec<u32> = split
            .next()
            .unwrap()
            .split(',')
            .filter_map(|v| v.parse::<u32>().ok())
            .collect();

        let boards: Vec<Vec<Vec<u32>>> = split
            .map(|p| {
                p.lines()
                    .map(|line| {
                        line.split(' ')
                            .filter_map(|v| v.parse::<u32>().ok())
                            .collect()
                    })
                    .collect()
            })
            .collect();

        let mut called = Vec::new();
        for call in calls {
            called.push(call);
            called.sort();

            for board in &boards {
                if is_board_won(&called, board) {
                    let board_score: u32 = board
                        .iter()
                        .map(|row| {
                            row.iter()
                                .filter(|v| called.binary_search(&v).is_err())
                                .sum::<u32>()
                        })
                        .sum();

                    return (call * board_score).into();
                }
            }
        }

        panic!()
    }

    fn part2(&self, input: Input) -> Output {
        let mut split = input.as_str().split("\n\n");
        let calls: Vec<u32> = split
            .next()
            .unwrap()
            .split(',')
            .filter_map(|v| v.parse::<u32>().ok())
            .collect();

        let boards: Vec<Vec<Vec<u32>>> = split
            .map(|p| {
                p.lines()
                    .map(|line| {
                        line.split(' ')
                            .filter_map(|v| v.parse::<u32>().ok())
                            .collect()
                    })
                    .collect()
            })
            .collect();

        let mut boards_won: Vec<_> = (0..(boards.len())).map(|_| false).collect();
        let mut win_count = 0;

        let mut called = Vec::new();
        for call in calls {
            called.push(call);
            called.sort();

            for (i, board) in boards.iter().enumerate() {
                if is_board_won(&called, board) {
                    let board_score: u32 = board
                        .iter()
                        .map(|row| {
                            row.iter()
                                .filter(|v| called.binary_search(&v).is_err())
                                .sum::<u32>()
                        })
                        .sum();

                    if !boards_won[i] {
                        boards_won[i] = true;
                        win_count += 1;

                        //Last board to win
                        if win_count == boards.len() {
                            return (call * board_score).into();
                        }
                    }
                }
            }
        }

        panic!()
    }
}
