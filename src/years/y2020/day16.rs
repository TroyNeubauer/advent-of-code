use std::collections::HashSet;

use crate::traits::*;

pub struct S;

#[derive(Debug)]
struct Constraint {
    name: String,
    a: std::ops::Range<u32>,
    b: std::ops::Range<u32>,
}

#[derive(Debug)]
struct Ticket(Vec<u32>);

#[derive(Debug)]
pub struct Data {
    constraints: Vec<Constraint>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn from_base_10_str(input: &str) -> Result<u32, std::num::ParseIntError> {
    std::str::FromStr::from_str(input)
}

fn int32(input: &str) -> nom::IResult<&str, u32> {
    nom::combinator::map_res(nom::character::complete::digit1, from_base_10_str)(input)
}

fn parse_constraint(input: &str) -> Result<Constraint, scanfmt::ScanError> {
    let (name, a1, a2, b1, b2): (_, _, u32, _, u32);
    scanfmt::scanfmt!(input, "{}: {}-{} or {}-{}", name, a1, a2, b1, b2);

    Ok(Constraint {
        name,
        a: std::ops::Range {
            start: a1,
            end: a2 + 1u32,
        },
        b: std::ops::Range {
            start: b1,
            end: b2 + 1u32,
        },
    })
}

mod part1 {
    use super::*;

    pub fn parse(input: &str) -> Data {
        let mut sections = input.split("\n\n");
        let constraints: Vec<_> = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| parse_constraint(line).unwrap())
            .collect();

        let ticket_part = sections.next().unwrap();
        let my_ticket = Ticket(
            ticket_part
                .lines()
                .nth(1)
                .unwrap()
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect(),
        );

        let other: Vec<Ticket> = sections
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| {
                Ticket(
                    line.split(',')
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect(),
                )
            })
            .collect();

        Data {
            constraints,
            my_ticket,
            other_tickets: other,
        }
    }
}

mod part2 {
    use super::*;

    pub fn parse(input: &str) -> Data {
        let mut sections = input.split("\n\n");
        let constraints: Vec<_> = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| parse_constraint(line).unwrap())
            .collect();

        let ticket_part = sections.next().unwrap();
        let my_ticket = Ticket(
            ticket_part
                .lines()
                .nth(1)
                .unwrap()
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect(),
        );

        let mut other_lines = sections.next().unwrap().lines();
        let _ = other_lines.next();

        let mut other: Vec<Ticket> = other_lines
            .map(|line| {
                Ticket(
                    line.split(',')
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect(),
                )
            })
            .collect();

        let mut i = 0;
        while i < other.len() {
            let mut valid = false;
            for value in &other[i].0 {
                valid = false;
                for con in &constraints {
                    if con.a.contains(value) || con.b.contains(value) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                i += 1;
            } else {
                other.remove(i);
            }
        }

        Data {
            constraints,
            my_ticket,
            other_tickets: other,
        }
    }
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let data = part1::parse(input.as_str());
        let mut bad_sum = 0;
        for ticket in &data.other_tickets {
            for value in &ticket.0 {
                let mut invalid = true;
                for con in &data.constraints {
                    if con.a.contains(value) || con.b.contains(value) {
                        invalid = false;
                    }
                }
                if invalid {
                    bad_sum += value;
                }
            }
        }
        bad_sum.into()
    }

    fn part2(&self, input: Input) -> Output {
        let data = part2::parse(input.as_str());

        //let con_names: HashMap<u32, &str>  = HashMap::new();
        //Stores the possible field id for each field index
        let mut possible: Vec<HashSet<u32>> = Vec::new();
        for _i in 0..data.my_ticket.0.len() {
            let mut set = HashSet::new();
            for id in 0..data.my_ticket.0.len() as u32 {
                set.insert(id);
            }

            possible.push(set);
        }

        for ticket in &data.other_tickets {
            for field_id in 0..data.constraints.len() as u32 {
                let con = &data.constraints[field_id as usize];
                for field_index in 0..ticket.0.len() as u32 {
                    let value = &ticket.0[field_index as usize];
                    if !con.a.contains(value) && !con.b.contains(value) {
                        possible[field_index as usize].remove(&field_id);
                    }
                }
            }
        }
        loop {
            //leave once all the fields know what they are...
            let mut ok = true;
            for set in &possible {
                if set.len() != 1 {
                    ok = false;
                    break;
                }
            }
            if ok {
                break;
            }

            for i in 0..possible.len() {
                let field_poss = &possible[i].clone();
                if field_poss.len() == 1 {
                    let known_value = &field_poss.iter().next().unwrap();
                    #[allow(clippy::needless_range_loop)]
                    for j in 0..possible.len() {
                        if i == j {
                            continue;
                        }
                        possible[j].remove(known_value);
                    }
                }
            }
        }

        let mut result = 1;
        for (i, one_value) in possible.into_iter().enumerate() {
            let actual_field_id = one_value.iter().next().unwrap();
            let final_value = data.my_ticket.0[i];
            let name = &data.constraints[*actual_field_id as usize].name;
            if name.starts_with("departure") {
                result *= final_value as usize;
            }
        }

        result.into()
    }
}
