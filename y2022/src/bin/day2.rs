#![allow(unused_variables, unused_imports)]
use itertools::*;
use util::{runner_main, AocDay};

struct Day2;

impl AocDay for Day2 {
    fn part1(&self, input: util::Input) -> util::Output {
        rounds(input.as_str())
            .map(|r| r.score())
            .sum::<i32>()
            .into()
    }

    fn part2(&self, input: util::Input) -> util::Output {
        rounds(input.as_str())
            .map(|r| r.to_expected().score())
            .sum::<i32>()
            .into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn parse(c: u8) -> Self {
        match c {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissors,
            _ => panic!(),
        }
    }

    fn is_win(self, other: Hand) -> bool {
        use Hand::*;

        match (self, other) {
            (Rock, Scissors) => true,
            (Paper, Rock) => true,
            (Scissors, Paper) => true,
            (_, _) => false,
        }
    }

    fn get_win(self) -> Self {
        use Hand::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
    fn get_loss(self) -> Self {
        use Hand::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Round {
    me: Hand,
    opp: Hand,
}

impl Round {
    fn score(&self) -> i32 {
        let shape = match self.me {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
        let result = if self.me == self.opp {
            3
        } else if self.me.is_win(self.opp) {
            // we won
            6
        } else {
            // we lost
            0
        };
        shape + result
    }

    fn to_expected(self) -> Self {
        use Hand::*;
        let to_play = match self.me {
            Rock => self.opp.get_win(),
            Paper => self.opp,
            Scissors => self.opp.get_loss(),
        };
        Self {
            opp: self.opp,
            me: to_play,
        }
    }
}

#[test]
fn a() {
    use Hand::*;
    assert_eq!(
        Round {
            opp: Hand::parse(b'A'),
            me: Hand::parse(b'Y'),
        }
        .score(),
        8
    );
    assert_eq!(
        Round {
            opp: Hand::parse(b'B'),
            me: Hand::parse(b'X'),
        }
        .score(),
        1
    );
    assert_eq!(
        Round {
            opp: Hand::parse(b'C'),
            me: Hand::parse(b'Z'),
        }
        .score(),
        6
    );
}

#[test]
fn b() {
    use Hand::*;
    assert_eq!(
        Round {
            opp: Hand::parse(b'A'),
            me: Hand::parse(b'Y'),
        }
        .to_expected()
        .score(),
        4
    );
    assert_eq!(
        Round {
            opp: Hand::parse(b'B'),
            me: Hand::parse(b'X'),
        }
        .to_expected()
        .score(),
        1
    );
    assert_eq!(
        Round {
            opp: Hand::parse(b'C'),
            me: Hand::parse(b'Z'),
        }
        .to_expected()
        .score(),
        7
    );
}

fn rounds(input: &str) -> impl Iterator<Item = Round> + '_ {
    input.lines().map(|l| Round {
        me: Hand::parse(l.as_bytes()[2]),
        opp: Hand::parse(l.as_bytes()[0]),
    })
}

fn main() {
    runner_main(&Day2, 2022, 2);
}
