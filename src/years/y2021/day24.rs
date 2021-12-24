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

struct Program(Vec<Ins>);

struct Computer([usize; 4]);

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

    fn is_valid(&mut self, program: &Program, serial: usize) -> bool {
        let base10 = serial.to_string();
        if base10.contains('0') {
            return false;
        }
        let mut s_index = 0;
        for ins in &program.0 {
            match ins {
                Ins::Inp(op) => {
                    self.write(op, (base10.as_bytes()[s_index] as usize) - (b'0' as usize));
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

        false
    }

    fn write(&mut self, reg: &Operand, value: usize) {
        match reg {
            Operand::Register(reg) => self.0[*reg as usize] = value,
            bad => panic!("Cannot write to non register {:?}", bad),
        }
    }

    fn read(&mut self, reg: &Operand) -> usize {
        match reg {
            Operand::Register(reg) => self.0[*reg as usize],
            Operand::Literal(lit) => *lit as usize,
        }
    }
}

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let program = Program::parse(input);
        let mut computer = Computer::new();
        let mut serial = 99_999_999_999_999;
        loop {
            if computer.is_valid(&program, serial) {
                break serial.into();
            }
            serial -= 1;
            println!("Trying {}", serial);
        }
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
