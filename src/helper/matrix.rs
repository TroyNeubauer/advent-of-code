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
    T: Clone,
{
    pub fn square(size: usize, val: T) -> Self {
        let mut data = Vec::new();
        for _ in 0..(size * size) {
            data.push(val.clone());
        }
        Self { data, cols: size }
    }
}

impl Matrix<u8> {
    pub fn new_from_chars(input: &str) -> Result<Self, String> {
        let mut rows = 0;
        let data: Vec<_> = input
            .lines()
            .map(|line| {
                rows += 1;
                line.bytes()
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
}

impl<T> Matrix<T> {
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

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        let rows = self.data.len() / self.cols;
        let cols = self.cols;
        if row >= rows || col >= cols {
            panic!();
        } else {
            let index = col + row * self.cols;
            // # Safety
            // 1. `row` is in range (0..rows)
            // 2. `col` is in range (0..cols)
            // 3  Therefore index will never exceed `col + `cols` * `rows`, which is the len of `self.data`
            //Some(unsafe { self.data.get_unchecked(index) })
            self.data[index] = val;
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

    pub fn rows(&self) -> usize {
        self.data.len() / self.cols
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

    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn neighbor_iter(&self, row: usize, col: usize) -> NeighborIter<'_, T> {
        NeighborIter {
            inner: NeighborEnumIter {
                mat: self,
                row,
                col,
                index: 0,
            },
        }
    }

    /// Returns an iter which gives (row_index, column_index, value) for the cells surrounding
    /// `row` and `col8
    pub fn enumerated_neighbor_iter(&self, row: usize, col: usize) -> NeighborEnumIter<'_, T> {
        NeighborEnumIter {
            mat: self,
            row,
            col,
            index: 0,
        }
    }

    pub fn print_with<F>(&self, f: impl Fn(&T) -> F)
    where
        F: std::fmt::Display,
    {
        for col in 0..self.cols() {
            for row in 0..self.rows() {
                print!("{}", f(self.get(row, col)));
            }
            println!();
        }
    }
}

impl<T> Matrix<T>
where
    T: std::fmt::Display,
{
    pub fn print(&self) {
        for col in 0..self.cols() {
            for row in 0..self.rows() {
                print!("{}", self.get(row, col));
            }
            println!();
        }
    }
}

pub struct EnumIter<'a, T> {
    data: &'a [T],
    index: usize,
    col_count: usize,
}

pub struct NeighborIter<'a, T> {
    inner: NeighborEnumIter<'a, T>,
}

pub struct NeighborEnumIter<'a, T> {
    mat: &'a Matrix<T>,
    row: usize,
    col: usize,
    index: usize,
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

impl<'a, T> Iterator for NeighborEnumIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        if self.index == OFFSETS.len() {
            None
        } else {
            let offset = OFFSETS[self.index];
            self.index += 1;
            let row = self.row as isize + offset.0;
            let col = self.col as isize + offset.1;

            //Skip ahead if we are out of bonds
            if row < 0 || row as usize >= self.mat.rows() {
                return self.next();
            }
            if col < 0 || col as usize >= self.mat.cols() {
                return self.next();
            }

            let row = row as usize;
            let col = col as usize;
            Some((row, col, self.mat.get(row, col)))
        }
    }
}

impl<'a, T> Iterator for NeighborIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((_, _, t)) => Some(t),
            None => None,
        }
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

    #[test]
    fn neighbor_iter() {
        let mat: Matrix<u32> = Matrix::new_delimitated("1 2 3\n4 5 6\n7 8 9", " ").unwrap();
        let sum: u32 = mat.neighbor_iter(1, 1).sum();
        assert_eq!(sum, 1 + 2 + 3 + 4 + 6 + 7 + 8 + 9);

        let sum: u32 = mat.neighbor_iter(0, 0).sum();
        assert_eq!(sum, 2 + 4 + 5);

        let sum: u32 = mat.neighbor_iter(2, 2).sum();
        assert_eq!(sum, 5 + 6 + 8);
    }
}
