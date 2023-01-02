use std::str::FromStr;

/// Simple parser interface for creating AOC style recursive decent parsers
pub struct Parser<'s> {
    inner: &'s str,
}

impl<'s> Parser<'s> {
    pub fn new(s: &'s str) -> Self {
        Self { inner: s }
    }

    pub fn next(&mut self) -> u8 {
        let b = self.inner.as_bytes()[0];
        self.inner = &self.inner[1..];
        b
    }

    pub fn last(&mut self) -> u8 {
        let last = self.inner.len() - 1;
        let b = self.inner.as_bytes()[last];
        self.inner = &self.inner[..last];
        b
    }

    pub fn next_char(&mut self) -> char {
        let b = self.inner.chars().next().unwrap();
        self.inner = &self.inner[b.len_utf8()..];
        b
    }

    pub fn next_number<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        let s = self.advance_to(|c| !c.is_ascii_digit());
        s.parse().unwrap()
    }

    /// Advances in the string until `f` returns true, returning the substring between the start
    /// and the character one before `f` returned true on
    pub fn advance_to(&mut self, f: impl Fn(u8) -> bool) -> &str {
        for (i, c) in self.inner.bytes().enumerate() {
            if f(c) {
                return self.advance(i);
            }
        }
        // Return rest
        self.advance(self.inner.len())
    }

    pub fn advance(&mut self, count: usize) -> &str {
        let (s, rest) = self.inner.split_at(count);
        self.inner = rest;
        return s;
    }

    pub fn expect(&mut self, expected: u8) {
        let s = self.inner;
        let next = self.next();
        if next != expected {
            panic!(
                "expected {}, got {}, in {s}",
                expected as char, next as char
            );
        }
    }

    pub fn expect_last(&mut self, expected: u8) {
        let s = self.inner;
        let next = self.last();
        if next != expected {
            panic!(
                "expected last {}, got {}, in {s}",
                expected as char, next as char
            );
        }
    }

    pub fn peek(&self) -> u8 {
        self.inner.as_bytes()[0]
    }

    pub fn try_peek(&self) -> Option<u8> {
        self.inner.as_bytes().get(0).copied()
    }

    /// Consumes the next character if it matches `pattern`.
    /// If self is empty or the character is not `pattern`, self is unchanged and Err(()) is
    /// returned
    pub fn try_consume(&mut self, pattern: impl Into<char>) -> Result<(), ()> {
        let pattern = pattern.into();
        match self.inner.chars().next() {
            Some(c) if c == pattern => {
                let _ = self.next_char();
                Ok(())
            }
            Some(_) => Err(()),
            None => Err(()),
        }
    }

    pub fn peek_is_digit(&self) -> bool {
        self.peek_is(|c| c.is_ascii_digit())
        //self.try_peek().map(|c| c.is_ascii_digit()).unwrap_or(false)
    }

    pub fn peek_is(&self, f: impl Fn(u8) -> bool) -> bool {
        self.try_peek().map(f).unwrap_or(false)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_str(&self) -> &str {
        self.inner
    }
}
