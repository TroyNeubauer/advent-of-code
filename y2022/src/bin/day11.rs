use itertools::Itertools;
use std::collections::HashMap;
use util::{runner_main, AocDay, Input, Output};

struct Day11;

#[derive(Debug)]
enum BoredOperation {
    Add(u32),
    Mutiply(u32),
    MutiplyOld,
}

#[derive(Debug)]
struct Monkey<I: std::fmt::Debug> {
    items: Vec<I>,
    operation: BoredOperation,
    test_divisible: u32,
    throw_if_true: usize,
    throw_if_false: usize,
}

type Monkey1 = Monkey<u32>;
type Monkey2 = Monkey<Item>;

#[derive(Debug)]
struct Item {
    /// Maps a modular base (N) to the current value (mod N)
    mods: HashMap<u32, u32>,
    initial_value: u32,
}

impl Item {
    fn init_base(&mut self, base: u32) {
        self.mods.insert(base, self.initial_value % base);
    }

    fn add(&mut self, rhs: u32) {
        for (base, val) in self.mods.iter_mut() {
            *val = (*val + rhs) % base;
        }
    }

    fn mul(&mut self, rhs: u32) {
        for (base, val) in self.mods.iter_mut() {
            *val = (*val * (rhs % base)) % base;
        }
    }

    fn square(&mut self) {
        for (base, val) in self.mods.iter_mut() {
            *val = (*val * *val) % base;
        }
    }
}

impl From<u32> for Item {
    fn from(value: u32) -> Self {
        Self {
            mods: HashMap::new(),
            initial_value: value,
        }
    }
}

fn parse<I>(i: &str) -> Vec<Monkey<I>>
where
    I: std::fmt::Debug + From<u32>,
{
    let mut it = i.lines();
    let mut monkeys = vec![];
    while let Some(_header) = it.next() {
        let items = it.next().unwrap();
        let items = items.split_once(':').unwrap().1;
        let items: Vec<I> = items
            .split(',')
            .map(|s| s.trim())
            .map(|m| m.parse().unwrap())
            .map(|l: u32| l.into())
            .collect();

        let operation = it.next().unwrap();

        let operation = if operation.contains("old * old") {
            BoredOperation::MutiplyOld
        } else {
            let mut operation_split = operation.split(' ').rev().take(2);
            let operation_value: u32 = operation_split.next().unwrap().parse().unwrap();
            let operation = operation_split.next().unwrap();

            match operation {
                "*" => BoredOperation::Mutiply(operation_value),
                "+" => BoredOperation::Add(operation_value),
                _ => panic!(),
            }
        };

        let test_value: u32 = it
            .next()
            .unwrap()
            .split(' ')
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let true_throw: usize = it
            .next()
            .unwrap()
            .split(' ')
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let false_throw: usize = it
            .next()
            .unwrap()
            .split(' ')
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let _empty_line = it.next();

        monkeys.push(Monkey {
            items,
            operation,
            test_divisible: test_value,
            throw_if_true: true_throw,
            throw_if_false: false_throw,
        });
    }
    monkeys
}

impl AocDay for Day11 {
    fn part1(&self, i: Input) -> Output {
        let mut monkeys: Vec<Monkey1> = parse(i.as_str());

        let mut monkey_activity: Vec<usize> = monkeys.iter().map(|_| 0).collect();

        for _round in 0..20 {
            for i in 0..monkeys.len() {
                for old in std::mem::take(&mut monkeys[i].items).into_iter() {
                    monkey_activity[i] += 1;
                    let level = match monkeys[i].operation {
                        BoredOperation::Add(v) => old + v,
                        BoredOperation::Mutiply(v) => old * v,
                        BoredOperation::MutiplyOld => old * old,
                    };
                    let level = level / 3;
                    let throw_to = if level % monkeys[i].test_divisible == 0 {
                        monkeys[i].throw_if_true
                    } else {
                        monkeys[i].throw_if_false
                    };
                    monkeys[throw_to].items.push(level);
                }
            }
        }
        let product: usize = monkey_activity
            .into_iter()
            .enumerate()
            .sorted_by_key(|(_i, c)| *c)
            .map(|(_i, c)| c)
            .rev()
            .take(2)
            .product();

        product.into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut monkeys: Vec<Monkey2> = parse(i.as_str());
        let bases: Vec<u32> = monkeys.iter().map(|m| m.test_divisible).collect();
        for base in bases {
            for monkey in &mut monkeys {
                for item in &mut monkey.items {
                    item.init_base(base);
                }
            }
        }
        let mut monkey_activity: Vec<usize> = monkeys.iter().map(|_| 0).collect();

        for _round in 0..10000 {
            for i in 0..monkeys.len() {
                for mut item in std::mem::take(&mut monkeys[i].items).into_iter() {
                    monkey_activity[i] += 1;
                    match monkeys[i].operation {
                        BoredOperation::Add(v) => item.add(v),
                        BoredOperation::Mutiply(v) => item.mul(v),
                        BoredOperation::MutiplyOld => item.square(),
                    };
                    let throw_to = if item.mods[&monkeys[i].test_divisible] == 0 {
                        monkeys[i].throw_if_true
                    } else {
                        monkeys[i].throw_if_false
                    };
                    monkeys[throw_to].items.push(item);
                }
            }
        }
        let product: usize = monkey_activity
            .into_iter()
            .enumerate()
            .sorted_by_key(|(_i, c)| *c)
            .map(|(_i, c)| c)
            .rev()
            .take(2)
            .product();

        product.into()
    }
}

fn main() {
    let d = Day11;
    runner_main(&d, 2022, 11);
}
