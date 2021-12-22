use crate::traits::*;

use combine::{
    error::{Commit, ParseError},
    parser::{
        char::{char, digit, spaces, string},
        choice::{choice, optional},
        repeat::{many, many1, sep_by},
        sequence::between,
        token::{any, satisfy, satisfy_map},
    },
    stream::{
        buffered,
        position::{self, SourcePosition},
        IteratorStream,
    },
    EasyParser, Parser, Stream, StreamOnce,
};

pub struct S;

#[derive(Debug, Clone)]
enum Number {
    Nested(Box<Number>, Box<Number>),
    Num(usize),
}

#[inline]
fn parse_snailfish_number<Input>() -> impl Parser<Input, Output = Number>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    parse_snailfish_number_()
}

combine::parser! {
    #[inline]
    fn parse_snailfish_number_[Input]()(Input) -> Number
        where [ Input: Stream<Token = char> ]
    {
        let array = between(
            char('['),
            char(']'),
            sep_by(parse_snailfish_number(), char(',')),
        ).map(Number::Rec);

        let number = many1(digit())
            .map(|s: String| {
                let mut n = 0;
                for c in s.chars() {
                    n = n * 10 + (c as usize - '0' as usize);
                }
                Number::Pair(n)
            })
            .expected("integer");

        choice((
            array,
            number,
        ))
    }
}

fn parse(input: Input) -> Vec<Number> {
    let mut lines = Vec::new();
    let mut parser = parse_snailfish_number();
    for line in input.lines() {
        let (number, _rest) = match parser.easy_parse(line) {
            Ok(n) => n,
            Err(err) => {
                println!("{:?}", err.position);
                panic!("{}", err);
            }
        };
        lines.push(number);
    }

    lines
}

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let input = parse(input);
        println!("{:?}", input);
        todo!()
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
