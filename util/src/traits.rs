use serde::{Deserialize, Serialize};
use std::str::{FromStr, Lines};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IsTest {
    Yes,
    No,
}

impl IsTest {
    pub fn is_test(&self) -> bool {
        matches!(self, IsTest::Yes)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Input(pub String, pub IsTest);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output(pub String);

pub trait AocDay {
    fn part1(&self, input: Input) -> Output;
    fn part2(&self, input: Input) -> Output;
}

impl Input {
    pub fn new(inner: String, is_test: IsTest) -> Self {
        Self(inner, is_test)
    }

    pub fn is_test(&self) -> bool {
        matches!(self.1, IsTest::Yes)
    }

    /// Returns an iterator over each line parsed as `T`
    ///
    /// # Panics
    /// This function will panic if any line fails to parse
    pub fn nums<T>(&self) -> impl Iterator<Item = T> + '_
    where
        T: FromStr + std::fmt::Debug,
    {
        self.lines().map(|line| match line.parse::<T>() {
            Ok(v) => v,
            Err(_e) => panic!("failed to parse `{}`", line),
        })
    }

    pub fn ints(&self) -> impl Iterator<Item = i32> + '_ {
        self.nums()
    }

    pub fn lines(&self) -> Lines {
        self.0.lines()
    }

    pub fn lines_bytes(&self) -> impl Iterator<Item = &'_ [u8]> + '_ {
        self.0.as_bytes().split(|&b| b == b'\n')
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Output {
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
