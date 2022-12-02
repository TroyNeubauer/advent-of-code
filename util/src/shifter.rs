pub struct Shifter<T, const N: usize> {
    values: [Option<T>; N],
    index: usize,
}

impl<T, const N: usize> Shifter<T, N> {
    pub fn new(current: T) -> Self {
        let mut values = [(); N].map(|_| None);
        values[0] = Some(current);
        Self { values, index: 0 }
    }

    pub fn empty() -> Self {
        Self {
            values: [(); N].map(|_| None),
            index: 0,
        }
    }

    pub fn get_current(&self) -> Option<&T> {
        match &self.values[0] {
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Adds a new element to this shifter, shifting last out and returning it
    pub fn shift(&mut self, new: T) -> Option<T> {
        self.index += 1;
        self.index %= N;
        let mut result = Some(new);
        std::mem::swap(&mut self.values[self.index], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifter1() {
        let mut shifter = Shifter::<usize, 3>::new(1);
        assert_eq!(*shifter.get_current().unwrap(), 1);
        assert_eq!(shifter.shift(2), None);
        assert_eq!(shifter.shift(3), None);
        assert_eq!(shifter.shift(4), Some(1));
        assert_eq!(shifter.shift(5), Some(2));
        assert_eq!(shifter.shift(6), Some(3));
        assert_eq!(shifter.shift(7), Some(4));
    }
}
