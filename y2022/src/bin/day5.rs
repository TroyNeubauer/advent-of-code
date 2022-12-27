use util::{runner_main, AocDay, Input, Output};

struct Day5;

impl AocDay for Day5 {
    fn part1(&self, input: Input) -> Output {
        let (mut yard, commands) = parse(input.as_str());
        for i in commands {
            yard.transfer(i);
        }
        yard.lanes
            .iter()
            .map(|stack| stack.get(stack.len() - 1).unwrap())
            .collect::<String>()
            .into()
    }

    fn part2(&self, input: Input) -> Output {
        let (mut yard, commands) = parse(input.as_str());
        for i in commands {
            yard.transfer_fixed(i);
        }
        yard.lanes
            .iter()
            .map(|stack| stack.get(stack.len() - 1).unwrap())
            .collect::<String>()
            .into()
    }
}

#[derive(Debug)]
struct Yard {
    lanes: Vec<Vec<char>>,
}

impl Yard {
    fn transfer(&mut self, i: Instruction) {
        for _ in 0..i.count {
            let src = i.src - 1;
            let dst = i.dst - 1;
            println!("moving {} from {} to {}", i.count, src, dst);
            let a = self.lanes[src].pop().unwrap();
            self.lanes[dst].push(a);
        }
        println!();
    }

    fn transfer_fixed(&mut self, i: Instruction) {
        let src = i.src - 1;
        let dst = i.dst - 1;
        println!("moving {} from {} to {}", i.count, src, dst);

        let index = self.lanes[src].len() - i.count;
        let end = self.lanes[src].split_off(index);
        self.lanes[dst].extend_from_slice(&end);
        println!();
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

fn parse(i: &str) -> (Yard, Vec<Instruction>) {
    let mut a = i.split("\n\n");
    let mut yard_lines = a.next().unwrap().lines().rev();
    let width = (yard_lines.next().unwrap().len() + 1) / 4;
    let mut yard = Yard {
        lanes: vec![Vec::new(); width],
    };
    for line in yard_lines {
        for i in 0..width {
            let c = line.chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                yard.lanes[i].push(c);
            }
        }
    }
    let instructions = a
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut count = 0;
            let mut src = 0;
            let mut dst = 0;

            scanf::sscanf!(line, "move {} from {} to {}", count, src, dst).unwrap();
            Instruction { count, src, dst }
        })
        .collect();

    (yard, instructions)
}

fn main() {
    runner_main(&Day5, 2022, 5);
}
