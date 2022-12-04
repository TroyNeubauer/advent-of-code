use serde::{Deserialize, Serialize};
use std::str::{FromStr, Lines};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Input(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output(pub String);

pub trait AocDay {
    fn part1(&self, input: Input) -> Output;
    fn part2(&self, input: Input) -> Output;
}

impl Input {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn nums_comma_separated<T>(&self) -> Vec<T>
    where
        T: FromStr,
    {
        self.0
            .trim()
            .split(',')
            .enumerate()
            .map(|(i, token)| match token.parse::<T>() {
                Ok(v) => v,
                Err(_err) => panic!("Failed to parse `{}` (num #{})", token, i),
            })
            .collect()
    }

    pub fn nums<T>(&self) -> impl Iterator<Item = T> + '_
    where
        T: FromStr,
    {
        self.lines().filter_map(|line| line.parse::<T>().ok())
    }

    pub fn ints(&self) -> impl Iterator<Item = i32> + '_ {
        self.lines().filter_map(|line| line.parse::<i32>().ok())
    }

    pub fn lines(&self) -> Lines {
        self.0.lines()
    }

    pub fn lines_bytes(&self) -> impl Iterator<Item = &'_ [u8]> + '_ {
        self.0.as_bytes().split(|&b| b == b'\n')
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<T> From<T> for Input
where
    T: ToString,
{
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}

impl Output {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<T> From<T> for Output
where
    T: ToString,
{
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}
