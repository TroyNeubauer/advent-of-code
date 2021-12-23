
use crate::traits::*;

pub struct S;

type Numbers = Vec<(u8, u8)>;

//Format (depth, value)
fn parse(input: Input) -> Vec<Numbers> {
    input
        .lines_bytes()
        .map(|line| {
            line.iter()
                .fold((0, Vec::new()), |(mut d, mut vec), c| {
                    match c {
                        b'[' => d += 1,
                        b']' => d -= 1,
                        b'0'..=b'9' => vec.push((d, c - b'0')),
                        b',' => {}
                        _ => unreachable!("{}", *c as char),
                    }
                    (d, vec)
                })
                .1
        })
        .collect()
}

fn add(mut a: Numbers, b: Numbers) -> Numbers {
    a.extend(b);
    a.iter_mut().for_each(|(d, _)| *d += 1);
    a
}

fn reduce(input: &mut Numbers, start: usize) {
    for i in start..input.len() - 1 {
        if input[i].0 == 5 {
            let (left, right) = (input[i].1, input[i + 1].1);
            input[i] = (4, 0);
            input.remove(i + 1);
            let _ = input.get_mut(i.overflowing_sub(1).0).map(|n| n.1 += left);
            let _ = input.get_mut(i + 1).map(|n| n.1 += right);
            return reduce(input, i);
        }
    }
    for i in 0..input.len() {
        let (d, n) = input[i];
        if n >= 10 {
            input[i] = (d + 1, n / 2);
            input.insert(i + 1, (d + 1, (n + 1) / 2));
            return reduce(input, i);
        }
    }
}

fn mag(i: &mut usize, depth: u8, sf: &Numbers) -> u16 {
    3 * if sf[*i].0 == depth {
        *i += 1;
        sf[*i - 1].1 as u16
    } else {
        mag(i, depth + 1, sf)
    } + 2 * if sf[*i].0 == depth {
        *i += 1;
        sf[*i - 1].1 as u16
    } else {
        mag(i, depth + 1, sf)
    }
}

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let input = parse(input);
        let mut it = input.into_iter();
        let mut a = it.next().unwrap();
        for b in it {
            a = add(a, b);
            reduce(&mut a, 0)
            
        }
        println!("got {:?}", a);
        mag(&mut 0, 1, &a).into()
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
