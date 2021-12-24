use std::sync::atomic::{Ordering, AtomicIsize};

use crate::traits::*;

pub struct S;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Register(u8),
    Literal(i8),
}

#[derive(Debug, Copy, Clone)]
enum Ins {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

#[derive(Debug, Clone)]
struct Program(Vec<Ins>);

struct Computer([isize; 4]);

impl Operand {
    fn parse<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let s = items.next().unwrap();
        match s {
            "w" => Operand::Register(0),
            "x" => Operand::Register(1),
            "y" => Operand::Register(2),
            "z" => Operand::Register(3),
            _ => Operand::Literal(s.parse().unwrap()),
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
                    "inp" => Ins::Inp(Operand::parse(&mut parts)),
                    "add" => Ins::Add(Operand::parse(&mut parts), Operand::parse(&mut parts)),
                    "mul" => Ins::Mul(Operand::parse(&mut parts), Operand::parse(&mut parts)),
                    "div" => Ins::Div(Operand::parse(&mut parts), Operand::parse(&mut parts)),
                    "mod" => Ins::Mod(Operand::parse(&mut parts), Operand::parse(&mut parts)),
                    "eql" => Ins::Eql(Operand::parse(&mut parts), Operand::parse(&mut parts)),
                    bad => unreachable!("{}", bad),
                }
            })
            .collect();

        Program(program)
    }
}

impl Computer {
    fn new() -> Self {
        Self([0; 4])
    }

    fn is_valid(&mut self, program: &Program, serial: isize) -> bool {
        let base10 = serial.to_string();
        if base10.contains('0') {
            return false;
        }
        let mut s_index = 0;
        for ins in &program.0 {
            match ins {
                Ins::Inp(op) => {
                    self.write(op, (base10.as_bytes()[s_index] as isize) - (b'0' as isize));
                    s_index += 1;
                }
                Ins::Add(op_a, op_b) => {
                    let a = self.read(op_a);
                    let b = self.read(op_b);
                    self.write(op_a, a + b);
                }
                Ins::Mul(op_a, op_b) => {
                    let a = self.read(op_a);
                    let b = self.read(op_b);
                    self.write(op_a, a * b);
                }
                Ins::Div(op_a, op_b) => {
                    let a = self.read(op_a);
                    let b = self.read(op_b);
                    self.write(op_a, a / b);
                }
                Ins::Mod(op_a, op_b) => {
                    let a = self.read(op_a);
                    let b = self.read(op_b);
                    self.write(op_a, a % b);
                }
                Ins::Eql(op_a, op_b) => {
                    let a = self.read(op_a);
                    let b = self.read(op_b);
                    let result = if a == b { 1 } else { 0 };
                    self.write(op_a, result);
                }
            }
        }

        self.0[3] == 0
    }

    fn write(&mut self, reg: &Operand, value: isize) {
        match reg {
            Operand::Register(reg) => self.0[*reg as usize] = value,
            bad => panic!("Cannot write to non register {:?}", bad),
        }
    }

    fn read(&mut self, reg: &Operand) -> isize {
        match reg {
            Operand::Register(reg) => self.0[*reg as usize],
            Operand::Literal(lit) => *lit as isize,
        }
    }
}

static SERIAL: AtomicIsize = AtomicIsize::new(100_000_000_000_000);

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let program = Program::parse(input);
        const NUM_THREADS: isize = 12;
        const STRIDE_LENGTH: isize = 1_000_000;
        let threads: Vec<_> = (0..NUM_THREADS)
            .map(|thread_id| {
                let program = program.clone();
                std::thread::spawn(move || {
                    let mut searched: isize = 0;
                    let mut computer = Computer::new();
                    loop {
                        let starting_serial = SERIAL.fetch_sub(STRIDE_LENGTH, Ordering::SeqCst);
                        if starting_serial < 1_000_000_000_000 {
                            println!("thread {} searched {}", thread_id, searched);
                            break;
                        }
                        for i in 0..STRIDE_LENGTH {
                            let serial = starting_serial - i;
                            if computer.is_valid(&program, serial) {
                                println!("Found valid thing: {}", serial);
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
            std::thread::sleep_ms(60000);

        }
        for thread in threads {
            thread.join().unwrap();
        }
        panic!("Check console for output");
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
