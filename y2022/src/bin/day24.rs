use std::collections::HashSet;

use itertools::Itertools;
use util::{runner_main, AocDay, Input, IntoEnumeratedCells, Matrix, Output};

struct Day24;

impl AocDay for Day24 {
    fn part1(&self, i: Input) -> Output {
        let (grid, start, end) = parse_input(i.as_str());
        let (states, cycle_start) = generate_all_states(grid.clone());
        solve(&states, cycle_start, 0, 0, start, grid.len() - 1, end).into()
    }

    fn part2(&self, i: Input) -> Output {
        let (grid, start, end) = parse_input(i.as_str());
        let (states, cycle_start) = generate_all_states(grid.clone());
        let fwd = solve(&states, cycle_start, 0, 0, start, grid.len() - 1, end);
        let bck = solve(&states, cycle_start, fwd, grid.len() - 1, end, 0, start);
        solve(&states, cycle_start, bck, 0, start, grid.len() - 1, end).into()
    }
}

fn main() {
    let d = Day24;
    runner_main(&d, 2022, 24);
}

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const MASK_LEFT: u8 = 0b0000_1000;
const MASK_RIGHT: u8 = 0b0000_0100;
const MASK_UP: u8 = 0b0000_0010;
const MASK_DOWN: u8 = 0b0000_0001;
const MASK_WALL: u8 = 0b1000_0000;

const STEP: &[(isize, isize)] = &[(-1, 0), (0, -1), (0, 0), (0, 1), (1, 0)];

pub fn parse_input(input: impl AsRef<str>) -> (Matrix<u8>, usize, usize) {
    let mut grid = Matrix::new_from_chars(input.as_ref()).unwrap();
    let start = 1;
    let end = grid.cols() - 2;

    for c in grid.iter_mut() {
        *c = match *c {
            b'<' => MASK_LEFT,
            b'>' => MASK_RIGHT,
            b'^' => MASK_UP,
            b'v' => MASK_DOWN,
            b'#' => MASK_WALL,
            b'.' => {
                // TODO: parse the actual start/end positions
                0
            }
            _ => panic!(),
        }
    }

    (grid, start, end)
}

fn next_state(grid: &Matrix<u8>) -> Matrix<u8> {
    let mut next = grid.clone();
    for c in next.iter_mut() {
        *c = 0;
    }
    let down = grid.rows() - 2;
    let right = grid.cols() - 2;

    for (r, c, &v) in grid.iter().enumerate_cells() {
        if v & MASK_WALL != 0 {
            *next.get_mut(r, c) |= MASK_WALL;
            continue;
        }

        if v & MASK_LEFT != 0 {
            let col = if c == 1 { right } else { c - 1 };
            *next.get_mut(r, col) |= MASK_LEFT;
        }

        if v & MASK_RIGHT != 0 {
            let col = if c == right { 1 } else { c + 1 };
            *next.get_mut(r, col) |= MASK_RIGHT;
        }

        if v & MASK_UP != 0 {
            let row = if r == 1 { down } else { r - 1 };
            *next.get_mut(row, c) |= MASK_UP;
        }

        if v & MASK_DOWN != 0 {
            let row = if r == down { 1 } else { r + 1 };
            *next.get_mut(row, c) |= MASK_DOWN;
        }
    }

    next
}

fn generate_all_states(initial: Matrix<u8>) -> (Vec<Matrix<u8>>, usize) {
    let mut states: HashMap<Matrix<u8>, usize> = HashMap::default();
    let mut state = initial;

    let cycle_start = loop {
        let next = next_state(&state);
        let order = states.len();

        let state_id = *states.entry(state).or_insert(order);
        if state_id != order {
            break state_id;
        }

        state = next;
    };

    let sequence = states
        .iter()
        .sorted_by_key(|(_mat, idx)| *idx)
        .map(|(mat, _)| mat.clone())
        .collect_vec();

    for i in sequence.iter().take(5) {
        i.print_as_chars();
    }

    (sequence, cycle_start)
}

fn manhattan(from: (usize, usize), to: (usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

pub fn solve(
    states: &[Matrix<u8>],
    cycle_start: usize,
    start_time: usize,
    rs: usize,
    cs: usize,
    re: usize,
    ce: usize,
) -> usize {
    let cycle_len = states.len() - cycle_start;

    let mut seen: HashSet<(usize, usize, usize)> = HashSet::default();
    let mut pq = BinaryHeap::new();
    pq.push((
        Reverse(manhattan((rs, cs), (re, ce))),
        Reverse(start_time),
        (rs, cs),
    ));

    while let Some((Reverse(_cost), Reverse(time), (r, c))) = pq.pop() {
        if r == re && c == ce {
            return time;
        }

        let state_idx = if time < cycle_start {
            time + 1
        } else {
            cycle_start + (time + 1 - cycle_start) % cycle_len
        };

        let state = &states[state_idx];

        for (dr, dc) in STEP.iter().copied() {
            let Some(rx) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(cx) = c.checked_add_signed(dc) else {
                continue;
            };
            if rx >= state.rows() || cx > state.cols() {
                continue;
            }

            if *state.get(rx, cx) == 0 && seen.insert((state_idx, rx, cx)) {
                let cost = manhattan((rx, cx), (re, ce)) + time + 1;
                pq.push((Reverse(cost), Reverse(time + 1), (rx, cx)));
            }
        }
    }

    panic!("no solution")
}
