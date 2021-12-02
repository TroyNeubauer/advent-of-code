use std::str::{FromStr, Lines};

pub type Error = Box<dyn std::error::Error>;

pub struct Input(String);
pub struct Output(String);

pub trait AocDay {
    fn part1(input: Input) -> Result<Output, Error>;
    fn part2(input: Input) -> Result<Output, Error>;
}

impl Input {
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
}

impl<T> From<T> for Input
where
    T: ToString,
{
    fn from(s: T) -> Self {
        Self(s.to_string())
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
