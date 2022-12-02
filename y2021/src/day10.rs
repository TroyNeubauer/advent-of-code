use crate::traits::*;

pub struct S;

fn matches(opener: char, closer: char) -> bool {
    opener == '(' && closer == ')'
        || opener == '[' && closer == ']'
        || opener == '{' && closer == '}'
        || opener == '<' && closer == '>'
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut count = 0;
        let mut characters = Vec::new();
        for line in input.lines() {
            //let mut wrong_char_type = None;
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        characters.push(c);
                    }
                    ')' | ']' | '}' | '>' => match characters.pop() {
                        Some(opener) => {
                            if !matches(opener, c) {
                                let points = match c {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    '>' => 25137,
                                    _ => unreachable!(),
                                };
                                count += points;
                            }
                        }
                        None => {
                            panic!("Unmatched {}", c);
                        }
                    },
                    _ => unreachable!(c),
                }
            }
        }
        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut count: usize = 0;
        let mut scores = Vec::new();
        'lines: for line in input.lines() {
            let mut characters = Vec::new();
            //let mut wrong_char_type = None;
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        characters.push(c);
                    }
                    ')' | ']' | '}' | '>' => match characters.pop() {
                        Some(opener) => {
                            if !matches(opener, c) {
                                continue 'lines;
                            }
                        }
                        None => {
                            panic!("Unmatched {}", c);
                        }
                    },
                    _ => unreachable!(c),
                }
            }
            for unmatched in characters.into_iter().rev() {
                count *= 5;
                let to_add = match unmatched {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                count += to_add;
            }
            scores.push(count);
            count = 0;
        }
        scores.sort();
        scores[scores.len() / 2].into()
    }
}
