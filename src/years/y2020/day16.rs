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
struct Ticket {
    fields: Vec<u32>,
}

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

fn parse_constraint(input: &str) -> nom::IResult<&str, Constraint> {
    let (input, (name, _, a1, _, a2, _, b1, _, b2, _)) = nom::sequence::tuple((
        nom::bytes::complete::take_until(":"),
        nom::bytes::complete::tag(": "),
        int32,
        nom::bytes::complete::tag("-"),
        int32,
        nom::bytes::complete::tag(" or "),
        int32,
        nom::bytes::complete::tag("-"),
        int32,
        nom::combinator::opt(nom::character::complete::line_ending),
    ))(input)?;

    Ok((
        input,
        Constraint {
            name: String::from(name),
            a: std::ops::Range {
                start: a1,
                end: a2 + 1,
            },
            b: std::ops::Range {
                start: b1,
                end: b2 + 1,
            },
        },
    ))
}

fn parse_constraints(input_raw: &str) -> (&str, Vec<Constraint>) {
    let mut result = Vec::new();
    let mut input = input_raw;

    //Parse constraints until we hit a double newline
    while !input.starts_with('\n') {
        let out = parse_constraint(input).expect("Failed to parse input");
        result.push(out.1);
        input = out.0;
    }

    (input, result)
}

fn parse_comment(input: &str) -> nom::IResult<&str, &str> {
    let (input, (_, _, comment, _)) = nom::sequence::tuple((
        nom::combinator::opt(nom::character::complete::line_ending),
        nom::combinator::opt(nom::character::complete::line_ending),
        nom::character::complete::not_line_ending,
        nom::character::complete::line_ending,
    ))(input)?;

    Ok((input, comment))
}

fn parse_ticket(input_raw: &str) -> nom::IResult<&str, Ticket> {
    let (input, (line, _)) = nom::sequence::tuple((
        nom::character::complete::not_line_ending,
        nom::character::complete::line_ending,
    ))(input_raw)?;

    Ok((
        input,
        Ticket {
            fields: line
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>())
                .map(|res| res.unwrap())
                .collect(),
        },
    ))
}

mod part1 {
    use super::*;

    pub fn parse(input: &str) -> Data {
        let (input, con) = parse_constraints(input);
        let (input, _comment1) = parse_comment(input).unwrap();
        let (input, my_ticket) = parse_ticket(input).unwrap();
        let (mut input, _comment2) = parse_comment(input).unwrap();

        let mut other: Vec<Ticket> = Vec::new();
        while !input.is_empty() {
            let (inner_input, ticket) = parse_ticket(input).unwrap();
            other.push(ticket);
            input = inner_input;
        }

        Data {
            constraints: con,
            my_ticket,
            other_tickets: other,
        }
    }
}

mod part2 {
    use super::*;

    pub fn parse(input: &str) -> Data {
        let (input, con) = parse_constraints(input);
        let (input, _comment1) = parse_comment(input).unwrap();
        let (input, my_ticket) = parse_ticket(input).unwrap();
        let (mut input, _comment2) = parse_comment(input).unwrap();

        let mut other: Vec<Ticket> = Vec::new();
        while !input.is_empty() {
            let (inner_input, ticket) = parse_ticket(input).unwrap();
            other.push(ticket);
            input = inner_input;
        }

        let mut i = 0;
        while i < other.len() {
            let mut valid = false;
            for value in &other[i].fields {
                valid = false;
                for con in &con {
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
            constraints: con,
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
            for value in &ticket.fields {
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
        for _i in 0..data.my_ticket.fields.len() {
            let mut set = HashSet::new();
            for id in 0..data.my_ticket.fields.len() as u32 {
                set.insert(id);
            }

            possible.push(set);
        }

        for ticket in &data.other_tickets {
            for field_id in 0..data.constraints.len() as u32 {
                let con = &data.constraints[field_id as usize];
                for field_index in 0..ticket.fields.len() as u32 {
                    let value = &ticket.fields[field_index as usize];
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
            let final_value = data.my_ticket.fields[i];
            let name = &data.constraints[*actual_field_id as usize].name;
            if name.starts_with("departure") {
                result *= final_value;
            }
        }

        result.into()
    }
}
