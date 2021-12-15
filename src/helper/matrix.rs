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

    pub fn new_from_single_nums(input: &str) -> Result<Self, String> {
        let mut rows = 0;
        let data: Vec<_> = input
            .lines()
            .map(|line| {
                rows += 1;
                line.bytes().map(|c| c - b'0')
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

    pub fn len(&self) -> usize {
        self.data.len()
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

    pub fn try_get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
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
            self.data.get_mut(index)
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

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let rows = self.data.len() / self.cols;
        let cols = self.cols;
        match self.try_get_mut(row, col) {
            Some(t) => t,
            None => {
                panic!(
                    "Matrix index out of range! row {}, col {}, rows {}, cols {}",
                    row, col, rows, cols
                );
            }
        }
    }

    /// Returns a mutable pointer to the element at `row`, `col`
    ///
    /// # Safety
    /// 1. The caller must guarantee that `row` in in range (0..self.rows())
    /// 2. The caller must guarantee that `col` in in range (0..self.cols())
    pub unsafe fn ptr_mut_unchecked(&mut self, row: usize, col: usize) -> *mut T {
        let index = col + row * self.cols;
        self.data.as_mut_ptr().add(index)
    }

    /// Returns a row major iterator over this matrix
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns a row major iterator over this matrix
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
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

    /// Returns an iter which gives (row_index, column_index, value) across the matrix
    pub fn cells(&self) -> Cells {
        Cells {
            index: 0,
            cols: self.cols,
            max_index: self.data.len(),
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

    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn neighbor_iter_mut(&mut self, row: usize, col: usize) -> NeighborIterMut<'_, T> {
        NeighborIterMut {
            inner: NeighborEnumIterMut {
                mat: self,
                row,
                col,
                index: 0,
            },
        }
    }

    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn strict_neighbor_iter(&self, row: usize, col: usize) -> StrictNeighborIter<'_, T> {
        StrictNeighborIter {
            inner: StrictNeighborEnumIter {
                mat: self,
                row,
                col,
                index: 0,
            },
        }
    }

    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn strict_enum_neighbor_iter(
        &self,
        row: usize,
        col: usize,
    ) -> StrictNeighborEnumIter<'_, T> {
        StrictNeighborEnumIter {
            mat: self,
            row,
            col,
            index: 0,
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

    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();
        Matrix {
            data,
            cols: self.cols,
        }
    }

    pub fn into_map<U, F>(self, f: F) -> Matrix<U>
    where
        F: Fn(T) -> U,
    {
        let data = self.data.into_iter().map(f).collect();
        Matrix {
            data,
            cols: self.cols,
        }
    }
}

impl<T> Matrix<T>
    where usize: From<T>,
          T: Clone
{

    pub fn pathfind(&self, start: (usize, usize), end: (usize, usize)) -> Option<(Vec<(usize, usize)>, usize)> {

        let successors = |pos: &(usize, usize)| -> Vec<((usize, usize), usize)> {
            let x = pos.0;
            let y = pos.1;
            let mut result = Vec::new();

            if x < end.0 {
                result.push(((x + 1, y), usize::from(self.get(x + 1, y).clone())));
            }

            if x > 0 {
                result.push(((x - 1, y), usize::from(self.get(x - 1, y).clone())));
            }

            if y < end.1 {
                result.push(((x, y + 1), usize::from(self.get(x, y + 1).clone())));
            }

            if y > 0 {
                result.push(((x, y - 1), usize::from(self.get(x, y - 1).clone())));
            }

            result
        };

        pathfinding::prelude::astar(
            &start,
            |p| successors(p),
            |p| {
                isize::abs(p.0 as isize - end.0 as isize) as usize + 
                isize::abs(p.1 as isize - end.1 as isize) as usize
            } / 3,
            |p| p.0 == end.0 && p.1 == end.1,
        )
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new_with_value(rows: usize, cols: usize, value: T) -> Matrix<T> {
        let mut data = Vec::new();
        for _ in 0..(rows * cols) {
            data.push(value.clone());
        }
        Matrix { data, cols }
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

pub struct Cells {
    index: usize,
    cols: usize,
    max_index: usize,
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

pub struct NeighborIterMut<'a, T> {
    inner: NeighborEnumIterMut<'a, T>,
}

pub struct NeighborEnumIterMut<'a, T> {
    mat: &'a mut Matrix<T>,
    row: usize,
    col: usize,
    index: usize,
}

pub struct StrictNeighborIter<'a, T> {
    inner: StrictNeighborEnumIter<'a, T>,
}

pub struct StrictNeighborEnumIter<'a, T> {
    mat: &'a Matrix<T>,
    row: usize,
    col: usize,
    index: usize,
}

impl Iterator for Cells {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.max_index {
            return None;
        }
        let col = self.index % self.cols;
        let row = self.index / self.cols;
        self.index += 1;
        Some((row, col))
    }
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
        const OFFSETS: [(i8, i8); 8] = [
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
            let row = self.row as isize + offset.0 as isize;
            let col = self.col as isize + offset.1 as isize;

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

impl<'a, T> Iterator for StrictNeighborEnumIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [(i8, i8); 4] = [(0, 1), (-1, 0), (1, 0), (0, -1)];

        if self.index == OFFSETS.len() {
            None
        } else {
            let offset = OFFSETS[self.index];
            self.index += 1;
            let row = self.row as isize + offset.0 as isize;
            let col = self.col as isize + offset.1 as isize;

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

impl<'a, T> Iterator for NeighborEnumIterMut<'a, T> {
    type Item = (usize, usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [(i8, i8); 8] = [
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
            let row = self.row as isize + offset.0 as isize;
            let col = self.col as isize + offset.1 as isize;

            //Skip ahead if we are out of bonds
            if row < 0 || row as usize >= self.mat.rows() {
                return self.next();
            }
            if col < 0 || col as usize >= self.mat.cols() {
                return self.next();
            }

            let row = row as usize;
            let col = col as usize;
            // Safety: row and col are in range by the bounds check above
            let ptr = unsafe { self.mat.ptr_mut_unchecked(row, col) };
            // Safety: mat is effectively split into multiple sub slices for each call to next here
            // that each live as long as this iterator lives. Because we borrow mat exclusively,
            // and do not touch the elements ourselves, creating this reference is safe here
            Some((row, col, unsafe { &mut *ptr }))
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

impl<'a, T> Iterator for StrictNeighborIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((_, _, t)) => Some(t),
            None => None,
        }
    }
}

impl<'a, T> Iterator for NeighborIterMut<'a, T> {
    type Item = &'a mut T;

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
