use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stored in row major ordering
    data: Vec<T>,
    x_len: usize,
    y_len: usize,
    z_len: usize,
}

impl<T, E> Matrix<T>
where
    T: FromStr,
    T: FromStr<Err = E>,
    E: std::fmt::Debug,
    T: Clone,
{
    pub fn cube(size: usize, val: T) -> Self {
        let mut data = Vec::new();
        for _ in 0..(size * size) {
            data.push(val.clone());
        }
        Self {
            data,
            x_len: size,
            y_len: size,
            z_len: size,
        }
    }
}

impl Matrix<u8> {
    /// Creates a new NxMx1 3d matrix from a 2d square of bytes
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
                "Non rectangular NxMx1 matrix. rows: {}, size: {}",
                rows,
                data.len()
            ));
        }

        let x_len = data.len() / rows;
        let y_len = rows;
        Ok(Self {
            data,
            x_len,
            y_len,
            z_len: 1,
        })
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

        let x_len = data.len() / rows;
        let y_len = rows;
        Ok(Self {
            data,
            x_len,
            y_len,
            z_len: 1,
        })
    }
}

impl<T> Matrix<T> {
    pub fn new_from_iterator(
        x_len: usize,
        y_len: usize,
        z_len: usize,
        it: impl Iterator<Item = T>,
    ) -> Self {
        let data: Vec<T> = it.collect();
        assert_eq!(data.len(), x_len * y_len * z_len);
        Self {
            data,
            x_len,
            y_len,
            z_len,
        }
    }

    pub fn cubes(&self) -> usize {
        self.data.len()
    }

    fn index(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= self.x_len || y >= self.y_len || z >= self.z_len {
            None
        } else {
            //We index by x's then y's then z's
            //This makes iterating by z -> y -> x fast because everything is sequential
            Some(x + y * self.y_len + z * self.y_len * self.z_len)
        }
    }

    fn force_index(&self, x: usize, y: usize, z: usize) -> usize {
        match self.index(x, y, z) {
            Some(index) => index,
            None => {
                panic!(
                    "Matrix index out of range! x {}, y {}, z {}, x_max {}, y_max {}, z_max {}",
                    x, y, z, self.x_len, self.y_len, self.z_len
                );
            }
        }
    }

    pub fn try_get(&self, x: usize, y: usize, z: usize) -> Option<&T> {
        self.index(x, y, z).map(|index| {
            // # Safety
            // 1. `x` is in range (0..self.x_len)
            // 2. `y` is in range (0..self.y_len)
            // 2. `z` is in range (0..self.z_len)
            // 3  Therefore `x + y * self.y_len + z * self.y_len * self.z_len`, will never exceed `self.data.len()`
            // (Bounds checked by `self.index()`)
            unsafe { self.data.get_unchecked(index) }
        })
    }

    pub fn try_get_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut T> {
        self.index(x, y, z).map(|index| {
            // Bounds checked by `self.index()`
            unsafe { self.data.get_unchecked_mut(index) }
        })
    }

    /// Gets a shared reference to the element at [x, y, z]
    ///
    /// # Panics if x or y or z are out of bounds
    pub fn get(&self, x: usize, y: usize, z: usize) -> &T {
        match self.try_get(x, y, z) {
            Some(t) => t,
            None => {
                panic!(
                    "Matrix index out of range! x {}, y {}, z {}, x_max {}, y_max {}, z_max {}",
                    x, y, z, self.x_len, self.y_len, self.z_len
                );
            }
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        // # Safety
        // Bounds checked by `self.force_index()`
        match self.try_get_mut(x, y, z) {
            Some(t) => t,
            None => {
                panic!(
                    "Matrix index out of range! x {}, y {}, z {}, x_max {}, y_max {}, z_max {}",
                    x, y, z, self.x_len, self.y_len, self.z_len
                );
            }
        }
    }

    /// Sets an element at [x, y, z] to val dropping the old value there
    ///
    /// # Panics if x or y or z are out of bounds
    pub fn set(&mut self, x: usize, y: usize, z: usize, val: T) {
        let dst = self.get_mut(x, y, z);
        *dst = val;
    }

    /// Returns a mutable pointer to the element at [x, y, z]
    ///
    /// # Safety
    /// 1. The caller must guarantee that `x` in in range (0..self.x_len())
    /// 2. The caller must guarantee that `y` in in range (0..self.y_len())
    /// 3. The caller must guarantee that `z` in in range (0..self.z_len())
    pub unsafe fn ptr_mut_unchecked(&mut self, x: usize, y: usize, z: usize) -> *mut T {
        // # Safety
        // The caller has guaranteed that x, and y, and z are in range so `self.index()` will never
        // return None
        let index = self
            .index(x, y, z)
            .unwrap_or(|| unsafe { std::hint::unreachable_unchecked!() });
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

    pub fn x_len(&self) -> usize {
        self.x_len
    }

    pub fn y_len(&self) -> usize {
        self.y_len
    }

    pub fn z_len(&self) -> usize {
        self.z_len
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

    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();
        Matrix {
            data,
            x_len: self.x_len,
            y_len: self.y_len,
            z_len: self.z_len,
        }
    }

    pub fn into_map<U, F>(self, f: F) -> Matrix<U>
    where
        F: Fn(T) -> U,
    {
        let data = self.data.into_iter().map(f).collect();
        Matrix {
            data,
            x_len: self.x_len,
            y_len: self.y_len,
            z_len: self.z_len,
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new_with_value(x_len: usize, y_len: usize, z_len: usize, value: T) -> Matrix<T> {
        let mut data = Vec::new();
        for _ in 0..(x_len * y_len * z_len) {
            data.push(value.clone());
        }
        Matrix {
            data,
            x_len,
            y_len,
            z_len,
        }
    }
}

impl<T> Matrix<T>
where
    T: std::fmt::Display,
{
    /// Prints a certain z level
    pub fn print(&self, z: usize) {
        for y in 0..self.y_len() {
            for x in 0..self.x_len() {
                print!("{}", self.get(x, y, z));
            }
            println!();
        }
    }
}

enum IterationMode {
    All,
    ZSlice(usize),
    StrictNeighbors { x: usize, y: usize, z: usize },
    Neighbors { x: usize, y: usize, z: usize },
}

/// Iterates through coordinates based on the mode selected
/// This serves as a low level utility that gathers the common functionality of the public
/// iterators in one place
struct CoordIterator {
    mode: IterationMode,
    x_len: usize,
    y_len: usize,
    z_len: usize,
    word: usize,
}

/// Converts a implementation defined state word to a set of coordinates based on the mode
impl Iterator for CoordIterator {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let coord = match self.mode {
            IterationMode::All => {
                let xy_len = self.x_len * self.y_len;
                let xyz_len = xy_len * self.z_len;
                if self.word >= xyz_len {
                    return None;
                }
                let xy = self.word % xy_len;
                let z = self.word / xy_len;
                let y = xy / self.x_len;
                let x = xy % self.x_len;
                (x, y, z)
            }
            IterationMode::ZSlice(z) => {
                let max = self.x_len * self.y_len;
                if self.word >= max {
                    return None;
                }
                let x = self.word % self.y_len;
                let y = self.word / self.y_len;
                (x, y, z)
            }
            IterationMode::StrictNeighbors { x, y, z } => {
                const OFFSETS: [(i8, i8, i8); 6] = [
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 1, 0),
                    (0, -1, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                ];
                if self.word >= OFFSETS.len() {
                    return None;
                }
                let offset = OFFSETS[self.word];
                let x = x as isize + offset.0 as isize;
                let y = y as isize + offset.1 as isize;
                let z = z as isize + offset.2 as isize;

                if x < 0 || x >= self.x_len as isize {
                    self.word += 1;
                    return self.next();
                }

                if y < 0 || y >= self.y_len as isize {
                    self.word += 1;
                    return self.next();
                }
                if z < 0 || z >= self.z_len as isize {
                    self.word += 1;
                    return self.next();
                }
                (x as usize, y as usize, z as usize)
            }
            IterationMode::Neighbors { x, y, z } => {
                const OFFSETS: [(i8, i8, i8); 6] = [
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 1, 0),
                    (0, -1, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                ];
                if self.word >= OFFSETS.len() {
                    return None;
                }
                let offset = OFFSETS[self.word];
                let x = x as isize + offset.0 as isize;
                let y = y as isize + offset.1 as isize;
                let z = z as isize + offset.2 as isize;

                if x < 0 || x >= self.x_len as isize {
                    self.word += 1;
                    return self.next();
                }

                if y < 0 || y >= self.y_len as isize {
                    self.word += 1;
                    return self.next();
                }
                if z < 0 || z >= self.z_len as isize {
                    self.word += 1;
                    return self.next();
                }
                (x as usize, y as usize, z as usize)
            }
        };

        self.word += 1;

        Some(coord)
    }
}
