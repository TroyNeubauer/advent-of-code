use crate::traits::*;
use bitset_core::BitSet;

pub struct S;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Config {
    state: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    A,
    B,
    C,
    D,
    Empty,
}

impl Color {
    fn ordal(self) -> usize {
        match self {
            Color::Empty => 0,
            Color::A => 1,
            Color::B => 2,
            Color::C => 3,
            Color::D => 4,
        }
    }

    fn from_ordal(ordal: usize) -> Self {
        match ordal {
            0 => Color::Empty,
            1 => Color::A,
            2 => Color::B,
            3 => Color::C,
            4 => Color::D,
            _ => unreachable!(),
        }
    }

    fn cell_index(self) -> usize {
        match self {
            Color::A => A_OFFSET,
            Color::B => B_OFFSET,
            Color::C => C_OFFSET,
            Color::D => D_OFFSET,
            _ => panic!(),
        }
    }

    fn is_empty(self) -> bool {
        matches!(self, Color::Empty)
    }
}

const HALLWAY_LENGTH: usize = 11;
const A_OFFSET: usize = HALLWAY_LENGTH;
const B_OFFSET: usize = A_OFFSET + 2;
const C_OFFSET: usize = B_OFFSET + 2;
const D_OFFSET: usize = C_OFFSET + 2;
const STATE_LENGTH: usize = D_OFFSET + 2;

impl Config {
    fn new() -> Self {
        Self { state: 0 }
    }

    fn parse(input: &str) -> Self {
        let mut state = Self::new();
        fn char_to_color(c: &str) -> Option<Color> {
            match c {
                "." => Some(Color::Empty),
                "A" => Some(Color::A),
                "B" => Some(Color::B),
                "C" => Some(Color::C),
                "D" => Some(Color::D),
                _ => None,
            }
        }
        let mut lines = input.lines();
        let _ = lines.next().unwrap();
        let hallway = &lines.next().unwrap()[1..12];
        let upper = &lines.next().unwrap()[3..10];
        let lower = &lines.next().unwrap()[3..10];
        for i in 0..HALLWAY_LENGTH {
            state.set(i, char_to_color(&hallway[i..i + 1]).unwrap());
        }
        for i in 0..7 {
            if let Some(color) = char_to_color(&upper[i..i + 1]) {
                state.set(HALLWAY_LENGTH + 1 + i, color);
            }
        }
        for i in 0..7 {
            if let Some(color) = char_to_color(&lower[i..i + 1]) {
                state.set(HALLWAY_LENGTH + i, color);
            }
        }
        let _ = lines.next().unwrap();
        state
    }

    fn is_solved(&self) -> bool {
        for i in 0..=HALLWAY_LENGTH {
            if !matches!(self.get(i), Color::Empty) {
                return false;
            }
        }
        matches!(self.get(A_OFFSET), Color::A)
            && matches!(self.get(A_OFFSET + 1), Color::A)
            && matches!(self.get(B_OFFSET), Color::B)
            && matches!(self.get(B_OFFSET + 1), Color::B)
            && matches!(self.get(C_OFFSET), Color::C)
            && matches!(self.get(C_OFFSET + 1), Color::C)
            && matches!(self.get(D_OFFSET), Color::D)
            && matches!(self.get(D_OFFSET + 1), Color::D)
    }

    fn get(&self, pos: usize) -> Color {
        if pos >= STATE_LENGTH {
            panic!("Out of bounds!");
        }
        let bits = self.state >> (pos * 3) & 0b111;
        Color::from_ordal(bits as usize)
    }

    fn set(&mut self, pos: usize, color: Color) {
        if pos >= STATE_LENGTH {
            panic!("Out of bounds!");
        }
        let offset = pos * 3;
        self.state &= !(0b111 << offset);
        self.state |= color.ordal() << offset;
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn get_char(color: Color) -> &'static str {
            match color {
                Color::Empty => ".",
                Color::A => "A",
                Color::B => "B",
                Color::C => "C",
                Color::D => "D",
            }
        }

        //Hallway
        f.write_str("#############\n")?;
        f.write_str("#")?;
        for i in 0..HALLWAY_LENGTH {
            f.write_str(get_char(self.get(i)))?;
        }
        f.write_str("#\n")?;

        //Top of holes
        f.write_str("###")?;

        f.write_str(get_char(self.get(A_OFFSET + 1)))?;
        f.write_str("#")?;
        f.write_str(get_char(self.get(B_OFFSET + 1)))?;
        f.write_str("#")?;
        f.write_str(get_char(self.get(C_OFFSET + 1)))?;
        f.write_str("#")?;
        f.write_str(get_char(self.get(D_OFFSET + 1)))?;

        f.write_str("###\n")?;

        //Bottom of holes
        f.write_str("  ")?;

        f.write_str("#")?;
        f.write_str(get_char(self.get(A_OFFSET)))?;
        f.write_str("#")?;
        f.write_str(get_char(self.get(B_OFFSET)))?;
        f.write_str("#")?;
        f.write_str(get_char(self.get(C_OFFSET)))?;
        f.write_str("#")?;
        f.write_str(get_char(self.get(D_OFFSET)))?;
        f.write_str("#")?;

        f.write_str("\n")?;

        //Bottom
        f.write_str("  #########")
    }
}
/*
fn solve(
    cfg: Config,
    cost: usize,
    costs: &mut Vec<usize>,
) {
    if cfg.is_solved() {
        return Some(cost);
    }

    let things = (0..STATE_LENGTH).filter_map(|i| {
        let c = cfg.get(i);
        if c == Color::Empty {
            None
        } else {
            Some((i, c))
        }
    });

    fn try_solution(cfg: Config, cost: usize, costs: &mut Vec<usize>, src: usize, dst: usize) {
        let critter = cfg.get(src);
        debug_assert!(cfg.get(dst).is_empty());
        cfg.set(dst, critter);
        cfg.set(src, Color::Empty);
        solve(cfg, cost + critter.cost
    }

    for (i, critter) in things {
        if i < HALLWAY_LENGTH {
            // We are in the hallway, so we can only move into our cell
            let index = critter.cell_index();
            if cfg.get(index).is_empty() {
                temp_locations.push(index);
            }
        } else {
        };
        println!("{:?} at {}", critter, i);
        let new_cfg = cfg.clone();
        for dest in temp_locations.clone().into_iter() {
            if let Some(cost) = solve(
        }
    }

    temp_costs.iter().min().copied()
}
*/

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        /*let cfg = Config::parse(input.as_str());
        println!("{}", cfg);
        let mut costs = Vec::new();
        solve(cfg, 0, &mut costs);
        costs.into_iter().min().unwrap().into()*/
        todo!()
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
