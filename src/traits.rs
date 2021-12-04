use std::str::{FromStr, Lines};

pub use std::collections::HashMap;
pub use crate::helper::*;

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Input(String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output(String);

pub trait AocDay {
    fn part1(&self, input: Input) -> Output;
    fn part2(&self, input: Input) -> Output;
}

impl Input {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn nums<T>(&self) -> Vec<T>
    where
        T: FromStr,
    {
        self.lines()
            .filter_map(|line| line.parse::<T>().ok())
            .collect()
    }

    pub fn lines(&self) -> Lines {
        self.0.lines()
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
