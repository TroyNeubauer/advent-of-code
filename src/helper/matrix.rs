use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stored in row major ordering
    data: Vec<T>,
    /// The number of columns (each row start this number apart)
    cols: usize,
}

impl<T, E> Matrix<T>
where
    T: FromStr,
    T: FromStr<Err = E>,
    E: std::fmt::Debug,
{
    pub fn new(input: &str) -> Result<Self, String> {
        Self::new_delimitated(input, " ")
    }

    pub fn new_delimitated(input: &str, delimiter: &str) -> Result<Self, String> {
        let mut rows = 0;
        let data: Vec<T> = input
            .lines()
            .map(|line| {
                rows += 1;
                line.split(delimiter).map(|s| T::from_str(s).unwrap())
            })
            .flatten()
            .collect();

        if data.len() % rows != 0 {
            return Err(format!(
                "Non rectangular matrix. rows: {}, size: {}",
                rows,
                data.len()
            ));
        }

        let cols = data.len() / rows;
        Ok(Self { data, cols })
    }

    pub fn new_from_iterator(cols: usize, it: impl Iterator<Item = T>) -> Self {
        let data: Vec<T> = it.collect();
        Self { data, cols }
    }

    pub fn try_get(&self, row: usize, col: usize) -> Option<&T> {
        let rows = self.data.len() / self.cols;
        let cols = self.cols;
        if row >= rows || col >= cols {
            None
        } else {
            let index = col + row * self.cols;
            // # Safety
            // 1. `row` is in range (0..rows)
            // 2. `col` is in range (0..cols)
            // 3  Therefore index will never exceed `col + `cols` * `rows`, which is the len of `self.data`
            //Some(unsafe { self.data.get_unchecked(index) })
            self.data.get(index)
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        match self.try_get(row, col) {
            Some(t) => t,
            None => {
                let rows = self.data.len() / self.cols;
                let cols = self.cols;
                panic!(
                    "Matrix index out of range! row {}, col {}, rows {}, cols {}",
                    row, col, rows, cols
                );
            }
        }
    }

    /// Returns a row major iterator over this matrix
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns an iter which gives (row_index, column_index, value) across the matrix
    pub fn enumerated_iter(&self) -> EnumIter<'_, T> {
        EnumIter {
            data: self.data.as_slice(),
            index: 0,
            col_count: self.cols,
        }
    }
}

pub struct EnumIter<'a, T> {
    data: &'a [T],
    index: usize,
    col_count: usize,
}

impl<'a, T> Iterator for EnumIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.data.len() {
            return None;
        }

        let i = self.index;
        self.index += 1;
        Some((i / self.col_count, i % self.col_count, &self.data[i]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_square() {
        let input = "a b c\nd e\nf g";
        assert!(Matrix::<char>::new(input).is_err());
    }
}
