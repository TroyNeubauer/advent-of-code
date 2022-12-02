use itertools::Itertools;

use crate::traits::*;

pub struct S;

#[derive(Debug)]
struct Data {
    starting: String,
    rules: Vec<(char, char, char)>,
}

fn parse(input: Input) -> Data {
    let p = input.into_inner();
    let mut p = p.split("\n\n");
    let start = p.next().unwrap().trim();
    let rest = p.next().unwrap();
    let mut vec = Vec::new();
    for line in rest.lines() {
        let mut p = line.split(" -> ");
        let a = p.next().unwrap();
        let c = p.next().unwrap();
        let mut chr = a.chars();
        let a = chr.next().unwrap();
        let b = chr.next().unwrap();
        vec.push((a, b, c.chars().next().unwrap()));
    }
    Data {
        starting: start.to_owned(),
        rules: vec,
    }
}

impl AocDay for S {
    fn part1(&self, input: crate::traits::Input) -> Output {
        let d = parse(input);
        let mut thing = d.starting.clone();
        for _ in 0..10 {
            let mut next = String::new();
            let mut first = true;
            'outer: for (a, b) in thing.chars().tuple_windows() {
                for (ra, rb, rc) in &d.rules {
                    if *ra == a && *rb == b {
                        if first {
                            next.push(a);
                        }
                        first = false;
                        next.push(*rc);
                        next.push(b);
                        continue 'outer;
                    }
                }
            }
            thing = next;
        }
        let mut freq: HashMap<char, usize> = HashMap::new();
        for c in thing.chars() {
            let a = freq.entry(c).or_default();
            *a += 1;
        }
        let max = freq.iter().map(|(k, v)| v).max().unwrap();
        let min = freq.iter().map(|(k, v)| v).min().unwrap();
        (*max - *min).into()
    }

    fn part2(&self, input: crate::traits::Input) -> Output {
        let d = parse(input);
        let mut counts: HashMap<(char, char), usize> = HashMap::new();
        let mut freq: HashMap<char, usize> = HashMap::new();
        for c in d.starting.chars() {
            let a = freq.entry(c).or_default();
            *a += 1;
        }

        for (a, b) in d.starting.chars().tuple_windows() {
            let f = counts.entry((a, b)).or_default();
            *f += 1;
        }
        for _ in 0..40 {
            let mut next_counts: HashMap<(char, char), usize> = HashMap::new();
            'outer2: for ((a, b), count) in &counts {
                for (ra, rb, rc) in &d.rules {
                    if ra == a && rb == b {
                        let first = next_counts.entry((*a, *rc)).or_default();
                        *first += count;
                        let second = next_counts.entry((*rc, *b)).or_default();
                        *second += count;

                        let letter_count = freq.entry(*rc).or_default();
                        *letter_count += count;
                        continue 'outer2;
                    }
                }
            }
            counts = next_counts;
        }

        let max = freq.iter().map(|(k, v)| *v).max().unwrap();
        let min = freq.iter().map(|(k, v)| *v).min().unwrap();
        (max - min).into()
    }
}
