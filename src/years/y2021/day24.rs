use std::collections::VecDeque;
use std::io::Write;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::traits::*;

pub struct S;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Register(u8);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Operand {
    Register(Register),
    Literal(i8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Ins {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InstructionOr {
    Ins(Ins),
    KnownValue(isize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SideEffectTree {
    ins: InstructionOr,
    index: usize,
    parents: HashMap<Register, SideEffectTree>,
}

#[derive(Debug, Clone)]
struct Program(Vec<Ins>);

struct Computer([isize; 4]);

impl Register {
    fn parse<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let s = items.next().unwrap();
        match s {
            "w" => Register(0),
            "x" => Register(1),
            "y" => Register(2),
            "z" => Register(3),
            _ => panic!("Unexpected {}", s),
        }
    }

    fn write(self, computer: &mut Computer, value: isize) {
        computer.0[self.0 as usize] = value;
    }

    fn read(self, computer: &Computer) -> isize {
        computer.0[self.0 as usize]
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            0 => "w",
            1 => "x",
            2 => "y",
            3 => "z",
            _ => unreachable!(),
        };
        f.write_str(c)
    }
}

impl Ins {
    fn get_modified_register(self) -> Register {
        match self {
            Ins::Inp(a) => a,
            Ins::Add(a, _b) => a,
            Ins::Mul(a, _b) => a,
            Ins::Div(a, _b) => a,
            Ins::Mod(a, _b) => a,
            Ins::Eql(a, _b) => a,
        }
    }

    fn get_read_write_registers(self) -> Vec<Register> {
        fn combine_args(a: Register, b: Operand) -> Vec<Register> {
            match b {
                Operand::Literal(_lit) => vec![a],
                Operand::Register(reg) => vec![a, reg],
            }
        }
        match self {
            Ins::Inp(a) => vec![a],
            Ins::Add(a, b) => combine_args(a, b),
            Ins::Mul(a, b) => combine_args(a, b),
            Ins::Div(a, b) => combine_args(a, b),
            Ins::Mod(a, b) => combine_args(a, b),
            Ins::Eql(a, b) => combine_args(a, b),
        }
    }
}

impl std::fmt::Display for Ins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ins::Inp(a) => f.write_fmt(format_args!("inp {}", a)),
            Ins::Add(a, b) => f.write_fmt(format_args!("add {} {}", a, b)),
            Ins::Mul(a, b) => f.write_fmt(format_args!("mul {} {}", a, b)),
            Ins::Div(a, b) => f.write_fmt(format_args!("div {} {}", a, b)),
            Ins::Mod(a, b) => f.write_fmt(format_args!("mod {} {}", a, b)),
            Ins::Eql(a, b) => f.write_fmt(format_args!("eql {} {}", a, b)),
        }
    }
}

impl std::fmt::Display for InstructionOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            InstructionOr::Ins(ins) => ins.fmt(f),
            InstructionOr::KnownValue(value) => value.fmt(f),
        }
    }
}

impl Operand {
    fn parse<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let s = items.next().unwrap();
        match s {
            "w" => Operand::Register(Register(0)),
            "x" => Operand::Register(Register(1)),
            "y" => Operand::Register(Register(2)),
            "z" => Operand::Register(Register(3)),
            _ => Operand::Literal(s.parse().unwrap()),
        }
    }

    fn read(self, computer: &Computer) -> isize {
        match self {
            Operand::Register(reg) => computer.0[reg.0 as usize],
            Operand::Literal(lit) => lit as isize,
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(lit) => f.write_fmt(format_args!("{}", lit)),
            Operand::Register(reg) => f.write_fmt(format_args!("{}", reg)),
        }
    }
}

impl Program {
    fn parse(input: Input) -> Program {
        let program = input
            .lines()
            .map(|line| {
                let mut parts = line.split(' ');
                let ins = parts.next().unwrap();
                match ins {
                    "inp" => Ins::Inp(Register::parse(&mut parts)),
                    "add" => Ins::Add(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "mul" => Ins::Mul(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "div" => Ins::Div(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "mod" => Ins::Mod(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "eql" => Ins::Eql(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    bad => unreachable!("{}", bad),
                }
            })
            .collect();

        Program(program)
    }

    /// Generates the side effect tree for the instruction at index, recursively
    fn build_side_effects_helper(
        &self,
        index: usize,
        min_index: usize,
        inp_positions: &[usize],
        possible_inputs: &mut [Vec<u8>],
    ) -> SideEffectTree {
        let ins = self.0[index];

        if let Ins::Mul(_reg, Operand::Literal(0)) = ins {
            //We know this is 0
            return SideEffectTree {
                ins: InstructionOr::KnownValue(0),
                parents: HashMap::new(),
                index,
            };
        }

        let mut result = SideEffectTree {
            ins: InstructionOr::Ins(ins),
            parents: HashMap::new(),
            index,
        };
        let all_regs = ins.get_read_write_registers();
        //println!("{} `{:?}`", ins, all_regs);
        for i in (min_index..index).rev() {
            //println!(" {} checked", i);
            //check if the register set in this instruction is one of the ones we care about
            let ins_candidate = self.0[i];
            let modified_register = ins_candidate.get_modified_register();
            if all_regs.contains(&modified_register) {
                if let std::collections::hash_map::Entry::Vacant(ent) =
                    result.parents.entry(modified_register)
                {
                    //println!("  {}", ins_candidate);
                    let side_effects = self.build_side_effects_helper(
                        i,
                        min_index,
                        inp_positions,
                        possible_inputs,
                    );
                    ent.insert(side_effects);
                }
            }
            if all_regs.len() == result.parents.len() {
                //println!("  breaking");
                //We found all the registers we care about
                break;
            }
        }
        result
    }

    fn build_side_effects(&self, max_depth: Option<usize>) -> (SideEffectTree, Vec<Vec<u8>>) {
        let input_indices: Vec<usize> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, ins)| {
                if let Ins::Inp(_reg) = ins {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let mut possible_inputs = vec![(0..10).into_iter().collect(); input_indices.len()];

        let side_effects = self.build_side_effects_helper(
            self.0.len() - 1,
            max_depth.map(|max| self.0.len() - max).unwrap_or(0),
            input_indices.as_slice(),
            possible_inputs.as_mut_slice(),
        );

        let mut next = VecDeque::new();
        next.push_back(&side_effects);

        println!("Before reduce: {}", side_effects.print_tree());
        while let Some(effect) = next.pop_front() {
            for (reg, e) in effect.parents.iter() {
                if effect.is_input().is_some() && e.parents.len() == 1 {
                    let input2 = e.parents.iter().next().unwrap().1;
                    if let Some(inp_reg2) = input2.is_input() {
                        if *reg == inp_reg2 {
                            //We found a useless input instruction because it is clobbered by
                            //another input.
                            //`input_indices` contains the indices of all input instructions,
                            //we can use the index in the program to find how many input instructions
                            //come before this one.
                            let input_2_index = input_indices.binary_search(&input2.index).unwrap();
                            possible_inputs[input_2_index].clear();
                            possible_inputs[input_2_index].push(0);
                            println!("Found useless: {}", input_2_index);
                        }
                    }
                }
                next.push_back(e);
            }
        }

        (side_effects.reduce(), possible_inputs)
        //(side_effects, possible_inputs)
    }

    fn write_to_rust_file(&self, name: &str, input_count: usize) -> std::io::Result<()> {
        let mut file = std::fs::File::create(name)?;
        writeln!(
            file,
            "{}",
            r#"
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

bool y2021_d24_execute(const uint8_t* input) {
     size_t w = 0;
     size_t x = 0;
     size_t y = 0;
     size_t z = 0;
"#
        )?;

        let mut input_index = 0;
        for ins in &self.0 {
            match ins {
                Ins::Inp(reg) => {
                    writeln!(file, "    {} = (size_t) input[{}];", reg, input_index)?;
                    input_index += 1;
                }
                Ins::Add(a, b) => writeln!(file, "    {} = {} + {};", a, a, b)?,
                Ins::Mul(a, b) => writeln!(file, "    {} = {} * {};", a, a, b)?,
                Ins::Div(a, b) => writeln!(file, "    {} = {} / {};", a, a, b)?,
                Ins::Mod(a, b) => writeln!(file, "    {} = {} % {};", a, a, b)?,
                Ins::Eql(a, b) => writeln!(
                    file,
                    "    if ({} == {}) {{ {} = 1; }} else {{ {} = 0; }};",
                    a, b, a, a
                )?,
            }
        }

        assert_eq!(input_count, input_index);

        writeln!(
            file,
            "{}",
            r#"
    return z == 0;
}
"#
        )?;

        Ok(())
    }
}

impl SideEffectTree {
    fn print_tree(&self) -> text_trees::StringTreeNode {
        let children = self.parents.iter().map(|(_reg, side)| side.print_tree());
        text_trees::StringTreeNode::with_child_nodes(self.ins.to_string(), children)
    }

    fn is_input(&self) -> Option<Register> {
        if let InstructionOr::Ins(Ins::Inp(reg)) = self.ins {
            Some(reg)
        } else {
            None
        }
    }

    fn reduce(mut self) -> Self {
        if let InstructionOr::Ins(Ins::Add(reg, operand)) = self.ins {
            if let Operand::Register(reg2) = operand {
                if let InstructionOr::KnownValue(0) = self.parents.get(&reg2).unwrap().ins {
                    let parent = self.parents.remove(&reg).unwrap();
                    //Anything plus a + 0 == a
                    self = parent;
                }
            }
        }
        let parents = std::mem::take(&mut self.parents);
        for (k, v) in parents {
            self.parents.insert(k, v.reduce());
        }
        self
    }
}

impl Computer {
    fn new() -> Self {
        Self([0; 4])
    }

    fn is_valid(&mut self, program: &Program, serial: &[u8]) -> bool {
        if serial.contains(&0) {
            return false;
        }
        let mut s_index = 0;
        for ins in &program.0 {
            match ins {
                Ins::Inp(op) => {
                    op.write(self, (serial[s_index] as isize) - (b'0' as isize));
                    s_index += 1;
                }
                Ins::Add(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a + b);
                }
                Ins::Mul(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a * b);
                }
                Ins::Div(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a / b);
                }
                Ins::Mod(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a % b);
                }
                Ins::Eql(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    let result = if a == b { 1 } else { 0 };
                    op_a.write(self, result);
                }
            }
        }

        self.0[3] == 0
    }
}

static SERIAL: AtomicIsize = AtomicIsize::new(99_999_999_999_999);

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let program = Program::parse(input);
        //let (side_effects, possible_inputs) = program.build_side_effects(Some(100));
        //println!("{}", side_effects.print_tree());
        let input_count = program
            .0
            .iter()
            .filter_map(|i| if let Ins::Inp(_) = i { Some(()) } else { None })
            .count();

        program.write_to_rust_file("out.c", input_count).unwrap();

        let gcc = std::process::Command::new("gcc")
            .arg("-shared")
            .arg("-o")
            .arg("out.so")
            .arg("out.c")
            .arg("-O3")
            .spawn()
            .unwrap();

        let code = gcc.wait_with_output().unwrap();
        if !code.status.success() {
            panic!(
                "Failed to run gcc: {}",
                String::from_utf8_lossy(code.stdout.as_slice())
            );
        }

        std::fs::remove_file("out.c").unwrap();
        let lib = Box::leak(Box::new(
            unsafe { libloading::Library::new("./out.so") }.unwrap(),
        ));
        let func: libloading::Symbol<fn(*const u8) -> bool> =
            unsafe { lib.get(b"y2021_d24_execute") }.unwrap();

        const NUM_THREADS: isize = 12;
        const STRIDE_LENGTH: isize = 1_000_000;
        const ACTUAL_STRIDE_LENGTH: isize = 9isize.pow(6);
        let threads: Vec<_> = (0..NUM_THREADS)
            .map(|_thread_id| {
                let func = func.clone();
                std::thread::spawn(move || {
                    let mut searched: isize = 0;
                    loop {
                        let starting_serial = SERIAL.fetch_sub(STRIDE_LENGTH, Ordering::SeqCst);
                        let base10 = starting_serial.to_string();
                        let mut serial = [0u8; 14];
                        for (i, c) in base10.bytes().enumerate() {
                            serial[i] = c;
                        }
                        if starting_serial < 1_000_000_000_000 {
                            break;
                        }

                        for _ in 0..ACTUAL_STRIDE_LENGTH {
                            let serial_index = serial.len() - 1;
                            sub(&mut serial, serial_index);
                            if func(serial.as_ptr()) {
                                println!(
                                    "Found valid thing: {:?}",
                                    serial.map(|c| (c + b'0') as char)
                                );
                                break;
                            }
                            searched += 1;
                        }
                    }
                })
            })
            .collect();

        let mut last = SERIAL.load(Ordering::Relaxed);
        while SERIAL.load(Ordering::Relaxed) >= 1_000_000_000_000 {
            let now = SERIAL.load(Ordering::Relaxed);
            println!("checked {} - {}", last - now, now);
            last = now;
            std::thread::sleep_ms(1000 * 60);
        }
        for thread in threads {
            thread.join().unwrap();
        }
        panic!("Check console for output");

        todo!()
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}

fn sub<const N: usize>(digits: &mut [u8; N], index: usize) {
    if digits[index] > 1 {
        digits[index] -= 1;
    } else {
        //We have to borrow
        if index == 0 {
            panic!("Would have negative result!");
        }
        sub::<N>(digits, index - 1);
        digits[index] = 9;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let mut test: [u8; 3] = [2, 1, 1];
        sub(&mut test, 2);
        assert_eq!(test, [1, 9, 9]);

        let mut test: [u8; 3] = [2, 2, 2];
        sub(&mut test, 2);
        assert_eq!(test, [2, 2, 1]);

        let mut test: [u8; 3] = [2, 1, 1];
        sub(&mut test, 2);
        assert_eq!(test, [1, 9, 9]);
    }

    #[test]
    fn reduce() {
        let program = r#"inp y
inp y
mul y 0
inp z
add z y"#;
        let program = Program::parse(Input::new(program.to_string()));
        let (e, inputs) = program.build_side_effects(None);
        let expected = Program::parse(Input::new("inp z".to_string()));
        let mut expected = expected.build_side_effects(None).0;
        expected.index = 3;

        assert_eq!(e, expected);

        let expected: Vec<Vec<u8>> = vec![vec![0], vec![0], (1..10).into_iter().collect()];
        //assert_eq!(inputs, expected);
    }
}
