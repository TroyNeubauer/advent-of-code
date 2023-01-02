use std::cmp::Ordering;

use util::{runner_main, AocDay, Input, Output, Parser};

struct Day13;

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(n) => n.fmt(f),
            Packet::List(l) => f.debug_list().entries(l).finish(),
        }
    }
}

fn parse(parser: &mut Parser) -> Packet {
    if parser.peek_is_digit() {
        Packet::Number(parser.next_number::<u32>())
    } else {
        parser.expect(b'[');
        let mut nums = vec![];
        while parser.try_consume(b']').is_err() {
            nums.push(parse(parser));
            let _ = parser.try_consume(b',');
        }

        Packet::List(nums)
    }
}

fn parse_lines(i: &str) -> Vec<(Packet, Packet)> {
    i.split("\n\n")
        .map(|parts| {
            let mut lines = parts.split('\n');
            let mut parser = Parser::new(lines.next().unwrap());
            let a = parse(&mut parser);

            let mut parser = Parser::new(lines.next().unwrap());
            let b = parse(&mut parser);
            (a, b)
        })
        .collect()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => {
                let l_len = l.len();
                let r_len = r.len();
                for (l, r) in l.into_iter().zip(r.into_iter()) {
                    let c = l.cmp(r);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                l_len.cmp(&r_len)
            }
            (Packet::List(_), Packet::Number(r)) => {
                self.cmp(&Packet::List(vec![Packet::Number(*r)]))
            }
            (Packet::Number(l), Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*l)]).cmp(other)
            }
        }
    }
}

impl AocDay for Day13 {
    fn part1(&self, i: Input) -> Output {
        let a = parse_lines(i.as_str());
        let mut result = 0;
        for (i, (l, r)) in a.into_iter().enumerate() {
            if l.cmp(&r) == Ordering::Less {
                result += i + 1;
            }
        }

        result.into()
    }

    fn part2(&self, i: Input) -> Output {
        let a = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
        let b = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

        let mut values = vec![a.clone(), b.clone()];
        for (a, b) in parse_lines(i.as_str()) {
            values.push(a);
            values.push(b);
        }
        values.sort();
        let a_ind = values.iter().position(|p| p == &a).unwrap();
        let b_ind = values.iter().position(|p| p == &b).unwrap();

        ((a_ind + 1) * (b_ind + 1)).into()
    }
}

fn main() {
    let d = Day13;
    runner_main(&d, 2022, 13);
}
