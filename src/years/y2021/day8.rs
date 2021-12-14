use crate::traits::*;
use itertools::Itertools;

pub struct S;

#[derive(Copy, Clone, Debug)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn to_segment(self) -> SevenSegment {
        match self {
            Digit::Zero => [1, 1, 1, 1, 1, 1, 0].into(),
            Digit::One => [0, 1, 1, 0, 0, 0, 0].into(),
            Digit::Two => [1, 1, 0, 1, 1, 0, 1].into(),
            Digit::Three => [1, 1, 1, 1, 0, 0, 1].into(),
            Digit::Four => [0, 1, 1, 0, 0, 1, 1].into(),
            Digit::Five => [1, 0, 1, 1, 0, 1, 1].into(),
            Digit::Six => [1, 0, 1, 1, 1, 1, 1].into(),
            Digit::Seven => [1, 1, 1, 0, 0, 0, 0].into(),
            Digit::Eight => [1, 1, 1, 1, 1, 1, 1].into(),
            Digit::Nine => [1, 1, 1, 0, 0, 1, 1].into(),
        }
    }

    fn all() -> [Digit; 10] {
        [
            Digit::Zero,
            Digit::One,
            Digit::Two,
            Digit::Three,
            Digit::Four,
            Digit::Five,
            Digit::Six,
            Digit::Seven,
            Digit::Eight,
            Digit::Nine,
        ]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct SevenSegment(pub [u8; 7]);

impl SevenSegment {
    fn from_mapping(digit: &str, mapping: &[char]) -> Self {
        let mut result = [0; 7];
        for c in digit.chars() {
            let mut index = None;
            for (i, m_c) in mapping.iter().enumerate() {
                if *m_c == c {
                    index = Some(i);
                }
            }
            result[index.unwrap()] = 1;
        }
        Self(result)
    }

    fn subtract(&self, other: &Self) -> Self {
        let mut i = 0;
        SevenSegment(self.0.map(|v| {
            let o = other.0[i];
            i += 1;
            v - o
        }))
    }

    fn jumble(&self, indices: &[u8]) -> Self {
        let mut result = [0; 7];
        for (i, mapping) in indices.iter().enumerate() {
            result[i] = self.0[*mapping as usize];
        }
        SevenSegment(result)
    }
}

impl From<[u8; 7]> for SevenSegment {
    fn from(parts: [u8; 7]) -> Self {
        SevenSegment(parts)
    }
}

#[derive(Clone, Debug)]
struct KnownDigit {
    digit: Digit,
    repr: String,
}

impl KnownDigit {
    fn new(digit: Digit, repr: String) -> Self {
        Self { digit, repr }
    }

    fn sub(&self, other: &Self) -> String {
        let mut s = String::new();
        for c in self.repr.chars() {
            let c_str = String::from(c);
            if !other.repr.contains(&c_str) {
                s.push_str(c_str.as_str());
            }
        }
        s
    }
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut count = 0;
        for line in input.lines() {
            let mut parts = line.split('|');
            let _first = parts.next().unwrap();
            let second = parts.next().unwrap();
            for part in second.split(' ') {
                if part.len() == 2 || part.len() == 4 || part.len() == 3 || part.len() == 7 {
                    count += 1;
                }
            }
        }

        count.into()
    }

    fn part2(&self, _input: Input) -> Output {
        panic!("Too hard");
    }
}

#[cfg(test)]
mod tests {
    use super::SevenSegment;

    #[test]
    fn test1() {
        let mapping = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let a = "efg";
        let s = SevenSegment::from_mapping(a, &mapping);
        assert_eq!(s.0, [0, 0, 0, 0, 1, 1, 1]);
    }
}
