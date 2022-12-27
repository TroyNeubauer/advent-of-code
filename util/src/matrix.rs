use std::{hash::Hash, num::TryFromIntError, str::FromStr};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stored in row major ordering
    data: Vec<T>,
    /// The number of columns (each row start this number apart)
    cols: usize,
    rows: usize,
}

impl<T, E> Matrix<T>
where
    T: FromStr<Err = E>,
    E: std::fmt::Debug,
    T: Clone,
{
    pub fn square(size: usize, val: T) -> Self {
        let mut data = Vec::new();
        for _ in 0..(size * size) {
            data.push(val.clone());
        }
        Self {
            data,
            cols: size,
            rows: size,
        }
    }
}

impl Matrix<u8> {
    pub fn new_from_chars(input: impl AsRef<str>) -> Result<Self, String> {
        let mut rows = 0;
        let input = input.as_ref();
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
                "Non square matrix. rows: {}, size: {}",
                rows,
                data.len()
            ));
        }

        let cols = data.len() / rows;
        Ok(Self { data, cols, rows })
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
                "Non square matrix. rows: {}, size: {}",
                rows,
                data.len()
            ));
        }

        let cols = data.len() / rows;
        Ok(Self { data, cols, rows })
    }

    pub fn print_as_chars(&self) {
        self.print_with(|c| *c as char);
    }

    pub fn print_as_chars_and_highlight_cell(
        &self,
        row: usize,
        col: usize,
        color: termcolor::Color,
    ) {
        self.print_with_and_highlight_cell(|c| *c as char, row, col, color);
    }

    pub fn print_as_chars_and_highlight<I>(&self, spec: HighlightSpec<I>)
    where
        I: Iterator<Item = (usize, usize)>,
    {
        self.print_with_and_highlight(|c| *c as char, spec);
    }

    /// Returns a matrix that contains all points
    pub fn from_relative_coords<I>(background_char: u8, it: I) -> Self
    where
        I: Iterator<Item = (u8, Point)> + Clone,
    {
        let min_row: usize = it.clone().map(|(_, p)| p.row).min().unwrap();
        let max_row: usize = it.clone().map(|(_, p)| p.row).max().unwrap();

        let min_col: usize = it.clone().map(|(_, p)| p.col).min().unwrap();
        let max_col: usize = it.clone().map(|(_, p)| p.col).max().unwrap();

        let rows = max_row - min_row + 1;
        let cols = max_col - min_col + 1;

        let mut result = Matrix::new_with_value(rows, cols, background_char);
        for (c, point) in it {
            result.set(point.row - min_row, point.col - min_col, c);
        }

        result
    }

    pub fn format_as_chars(&self) -> String {
        let mut s = String::new();
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                s.push(*self.get(row, col) as char);
            }
            if row != self.rows() - 1 {
                s.push('\n');
            }
        }
        s
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
                "Non square matrix. rows: {}, size: {}",
                rows,
                data.len()
            ));
        }

        let cols = data.len() / rows;
        Ok(Self { data, cols, rows })
    }
}

impl<T> Matrix<T> {
    pub fn new_from_iterator(cols: usize, it: impl Iterator<Item = T>) -> Self {
        let data: Vec<T> = it.collect();
        Self {
            data,
            cols,
            rows: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            false
        } else {
            true
        }
    }

    pub fn in_bounds_signed(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 || row >= self.rows as isize || col >= self.cols as isize {
            false
        } else {
            true
        }
    }

    /// Returns `Some((row, col))` if offsetting `row` by `row_offset` and `col` by `col_offset`
    /// produces absloute cell coordinates that are in range.
    ///
    /// If the resulting cell coordinates are out of bounds, `None` is returned
    pub fn offset(
        &self,
        row: usize,
        col: usize,
        row_offset: isize,
        col_offset: isize,
    ) -> Option<(usize, usize)> {
        let row = row as isize + row_offset;
        let col = col as isize + col_offset;
        if self.in_bounds_signed(row, col) {
            Some((row as usize, col as usize))
        } else {
            None
        }
    }

    /// Returns true if (row, col) is a cell which is on the edge of this matrix
    pub fn is_edge(&self, row: usize, col: usize) -> bool {
        let row_edge = row == 0 || row == self.rows() - 1;
        let col_edge = col == 0 || col == self.cols() - 1;
        row_edge || col_edge
    }

    /// Converts the cell location [row, col] to an index in the flat 1D array
    /// Always returns a valid index for `self.data` if [row, col] are in range
    fn index(&self, row: usize, col: usize) -> usize {
        col + row * self.cols
    }

    pub fn try_get(&self, row: usize, col: usize) -> Option<&T> {
        if !self.in_bounds(row, col) {
            None
        } else {
            // # Safety
            // `Self::in_bounds` returning true guarntees that calling `Self::index` will produce
            // an index that is in range for `self.data`
            // Some(unsafe { self.data.get_unchecked(index) })
            self.data.get(self.index(row, col))
        }
    }

    pub fn try_get_signed(&self, row: isize, col: isize) -> Option<&T> {
        if !self.in_bounds_signed(row, col) {
            None
        } else {
            // # Safety
            // `Self::in_bounds` returning true guarntees that calling `Self::index` will produce
            // an index that is in range for `self.data`
            // Some(unsafe { self.data.get_unchecked(index) })
            self.data.get(self.index(row as usize, col as usize))
        }
    }

    pub fn try_get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if !self.in_bounds(row, col) {
            None
        } else {
            let index = self.index(row, col);
            // # Safety
            // `Self::in_bounds` returning true guarntees that calling `Self::index` will produce
            // an index that is in range for `self.data`
            // Some(unsafe { self.data.get_unchecked(index) })
            self.data.get_mut(index)
        }
    }

    #[track_caller]
    pub fn set(&mut self, row: usize, col: usize, val: T) {
        if !self.in_bounds(row, col) {
            panic!();
        } else {
            let index = self.index(row, col);
            // # Safety
            // `Self::in_bounds` returning true guarntees that calling `Self::index` will produce
            // an index that is in range for `self.data`
            // Some(unsafe { self.data.get_unchecked(index) })
            self.data[index] = val;
        }
    }

    #[track_caller]
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

    #[track_caller]
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
    pub unsafe fn ptr_mut_at(&mut self, row: usize, col: usize) -> *mut T {
        unsafe { self.data.as_mut_ptr().add(self.index(row, col)) }
    }

    pub fn wrapping_get(&self, row: isize, col: isize) -> &T {
        let rows = self.rows();
        let cols = self.cols();
        let row = if row < 0 {
            row.rem_euclid(rows as isize) as usize
        } else {
            row as usize % rows
        };
        let col = col.rem_euclid(cols as isize) as usize;

        self.get(row, col)
    }

    /// Returns a row major iterator over this matrix
    pub fn iter(&self) -> IterRef<'_, T> {
        let col_count = self.cols();
        IterRef::new(self.data.iter(), col_count)
    }

    /// Returns a row major iterator over this matrix
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let col_count = self.cols();
        IterMut::new(self.data.iter_mut(), col_count)
    }

    /// Returns a row major iterator over this matrix
    pub fn into_iter(self) -> IterOwned<T> {
        let col_count = self.cols();
        IterOwned::new(self.data.into_iter(), col_count)
    }

    pub fn rows(&self) -> usize {
        self.data.len() / self.cols
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn neighbor_iter(&self, row: usize, col: usize) -> OffsetIterSlice<'_, '_, T, i8> {
        OffsetIterSlice::new(self, row, col, NEIGHBOR_OFFSETS.iter())
    }

    /*
    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn neighbor_iter_mut(&mut self, row: usize, col: usize) -> NeighborIterMut<'_, T> {
    }
    */

    /// Returns an iter which gives references to the values for the cells surrounding
    /// `row` and `col`
    pub fn adjacent_neighbor_iter(&self, row: usize, col: usize) -> OffsetIterSlice<'_, '_, T, i8> {
        OffsetIterSlice::new(self, row, col, ADJACENT_NEIGHBOR_OFFSETS.iter())
    }

    pub fn print_with<F>(&self, f: impl Fn(&T) -> F)
    where
        F: std::fmt::Display,
    {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                print!("{}", f(self.get(row, col)));
            }
            println!();
        }
    }

    pub fn print_with_and_highlight_cell<F>(
        &self,
        f: impl Fn(&T) -> F,
        row: usize,
        col: usize,
        color: termcolor::Color,
    ) where
        F: std::fmt::Display,
    {
        let spec = HighlightSpec {
            primary_row: row,
            primary_col: col,
            primary_color: Some(color),
            secondary_cells: [].into_iter(),
            secondary_color: None,
        };
        self.print_with_and_highlight(f, spec);
    }

    pub fn print_with_and_highlight<F, I>(&self, f: impl Fn(&T) -> F, spec: HighlightSpec<I>)
    where
        I: Iterator<Item = (usize, usize)>,
        F: std::fmt::Display,
    {
        use std::io::Write;
        use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};

        let stdout = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = stdout.buffer();

        stdout.print(&buffer).unwrap();

        // calculate secondary indices in order so we can
        let secondary: Vec<usize> = spec
            .secondary_color
            .map(|_| {
                let mut v: Vec<_> = spec
                    .secondary_cells
                    .map(|(row, col)| self.index(row, col))
                    .collect();
                v.sort_unstable();
                v
            })
            .unwrap_or(Vec::new());
        let mut secondary = secondary.into_iter().peekable();

        let primany_index = self.index(spec.primary_row, spec.primary_col);
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let index = self.index(row, col);
                let matched_primary = spec
                    .primary_color
                    .map(|c| {
                        if index == primany_index {
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .flatten();
                if let Some(color) = matched_primary {
                    buffer
                        .set_color(ColorSpec::new().set_fg(Some(color)))
                        .unwrap();
                }

                let matched_secondary = spec
                    .secondary_color
                    .map(|c| match secondary.peek().copied() {
                        Some(next_secondary) if next_secondary > index => {
                            // next secondary coming up in future
                            None
                        }
                        Some(next_secondary) if next_secondary == index => {
                            secondary.next().unwrap();
                            // we are on this next secondary. Consume and highlight
                            Some(c)
                        }
                        // nothing more to highlight
                        None => None,
                        // secondary is sorted, and we always consume when equal to current,
                        // so should always be >= current, unless the user is trolling us
                        _ => {
                            panic!("secondary highlight iterator returned mutiple of the same cell")
                        }
                    })
                    .flatten();
                if let Some(color) = matched_secondary {
                    if matched_primary.is_none() {
                        // change to secondary color if this cell is only secondary
                        buffer
                            .set_color(ColorSpec::new().set_fg(Some(color)))
                            .unwrap();
                    }
                }

                write!(buffer, "{}", f(self.get(row, col))).unwrap();
                if matched_primary.is_some() || matched_secondary.is_some() {
                    buffer.reset().unwrap();
                }
            }
            writeln!(buffer).unwrap();
        }
        stdout.print(&buffer).unwrap();
    }

    /// Returns the point of the first cell that matches `predicate`
    pub fn find<P>(&self, mut predicate: P) -> Option<Point>
    where
        P: FnMut(&T) -> bool,
    {
        self.iter()
            .enumerate_cells()
            .filter(|(_r, _c, v)| predicate(v))
            .map(|(r, c, _)| Point::new(r, c))
            .next()
    }

    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();
        Matrix {
            data,
            cols: self.cols,
            rows: self.rows,
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
            rows: self.rows,
        }
    }

    pub fn row(&self, row: usize) -> IterRef<'_, T> {
        let index = self.index(row, 0);
        let row_slice = &self.data[index..(index + self.cols())];
        IterRef::new(row_slice.iter(), self.cols())
    }

    pub fn first_row(&self) -> IterRef<'_, T> {
        self.row(0)
    }

    pub fn last_row(&self) -> IterRef<'_, T> {
        self.row(self.rows() - 1)
    }

    pub fn traverse(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> OffsetIter<T, VectorOffsetIter, isize> {
        if !self.in_bounds(row, col) {
            panic!();
        }
        let count = match direction {
            Direction::Up => row,
            Direction::Down => self.rows() - row - 1,
            Direction::Left => col,
            Direction::Right => self.cols() - col - 1,
        };
        let offsets = VectorOffsetIter::new(count, direction);
        OffsetIter {
            offsets: offsets.peekable(),
            mat: self,
            row,
            col,
            stop_on_out_of_bounds: true,
            done: false,
        }
    }

    /// Gets the coordinates corner that will result in traversing a single row or colum on the
    /// edge based on `mode`
    pub fn corner_for_traverse(&self, mode: Direction) -> (usize, usize) {
        match mode {
            Direction::Up => (self.rows() - 1, self.cols() - 1),
            Direction::Down => (0, 0),
            Direction::Left => (self.rows() - 1, self.cols() - 1),
            Direction::Right => (0, 0),
        }
    }

    /// Returns a direction that points toward the center based on the size of the matrix and
    /// `row`, `col`.
    ///
    /// Cells along diagionals have two possible directions, for example [0, 0] has no cardinal
    /// direction that points to the center, but [`Direction::Down`] and [`Direction::Right`] are
    /// "correct answers". Calling this function for any cell on a diagional will return either one
    /// of the two "correct" answers.
    pub fn direction_to_center(&self, row: usize, col: usize) -> Direction {
        if row > col {
            // below major diagional
            let row_from_bottom = self.rows() - 1 - row;
            if col > row_from_bottom {
                Direction::Up
            } else {
                Direction::Right
            }
        } else {
            // above major diagional
            let col_from_right = self.cols() - 1 - col;
            if col_from_right > row {
                Direction::Down
            } else {
                Direction::Left
            }
        }
    }
}

impl<T> Hash for Matrix<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.cols);
        self.data.hash(state);
    }
}

pub struct HighlightSpec<I>
where
    I: Iterator<Item = (usize, usize)>,
{
    pub primary_row: usize,
    pub primary_col: usize,
    pub primary_color: Option<termcolor::Color>,
    pub secondary_cells: I,
    pub secondary_color: Option<termcolor::Color>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the "opposite" dimension.
    /// Useful with traversing all cols in a row, or all rows in a col with
    /// [`Matrix::corner_for_traverse`] and [`Matrix::traverse`] programmatically
    ///
    /// `assert_eq!(mode.transpose().transpose().transpose().transpose(), mode);` holds true for all modes
    pub fn transpose(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn to_unit_offsets(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn offset_coords(&self, mut row: usize, mut col: usize) -> (usize, usize) {
        let dir = self.to_unit_offsets();
        row = row.checked_add_signed(dir.0).unwrap();
        col = col.checked_add_signed(dir.1).unwrap();
        (row, col)
    }

    pub fn direction_between(
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> Option<Direction> {
        if from_row == to_row {
            if let Some(to_col) = to_col.checked_sub(1) {
                if from_col == to_col {
                    return Some(Direction::Right);
                }
            }
            if let Some(from_col) = from_col.checked_sub(1) {
                if from_col == to_col {
                    return Some(Direction::Left);
                }
            }
        }
        if from_col == to_col {
            if let Some(to_row) = to_row.checked_sub(1) {
                if from_row == to_row {
                    return Some(Direction::Down);
                }
            }
            if let Some(from_row) = from_row.checked_sub(1) {
                if from_row == to_row {
                    return Some(Direction::Up);
                }
            }
        }
        None
    }
}

/// Converts from the ASCII bytes `b'l', b'L', b'r', b'R', b'u', b'U', b'd', b'D'` to the Up, Down,
/// Left, or Right directions
impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'l' | b'L' => Direction::Left,
            b'r' | b'R' => Direction::Right,
            b'u' | b'U' => Direction::Up,
            b'd' | b'D' => Direction::Down,
            _ => return Err(()),
        })
    }
}

/// Converts from characters `'l', 'L', 'r', 'R', 'u', 'U', 'd', 'D'` to the Up, Down,
/// Left, or Right directions
impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let b: u8 = value.try_into().map_err(|_| ())?;
        Direction::try_from(b)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn offset(self, direction: Direction) -> Self {
        let (row, col) = direction.offset_coords(self.row, self.col);
        Self::new(row, col)
    }
}

impl std::ops::Sub for Point {
    type Output = SignedPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        let a: SignedPoint = self.try_into().unwrap();
        let b: SignedPoint = rhs.try_into().unwrap();
        a - b
    }
}

impl std::ops::Add<SignedPoint> for Point {
    type Output = Self;

    fn add(self, rhs: SignedPoint) -> Self::Output {
        Self::new(
            self.row.checked_add_signed(rhs.row).unwrap(),
            self.col.checked_add_signed(rhs.col).unwrap(),
        )
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl std::ops::AddAssign<SignedPoint> for Point {
    fn add_assign(&mut self, rhs: SignedPoint) {
        *self = *self + rhs
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl TryFrom<Point> for SignedPoint {
    type Error = TryFromIntError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        Ok(Self {
            row: value.row.try_into()?,
            col: value.col.try_into()?,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedPoint {
    pub row: isize,
    pub col: isize,
}

impl SignedPoint {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn offset(self, direction: Direction) -> Self {
        let (row_delta, col_delta) = direction.to_unit_offsets();
        Self::new(self.row + row_delta, self.col + col_delta)
    }

    /// Returns true if this point has one zero axis and one non-zero axis
    pub fn is_axis_aligned(self) -> bool {
        let row_aligned = self.row == 0 && self.col != 0;
        let col_aligned = self.row != 0 && self.col == 0;
        row_aligned || col_aligned
    }

    pub fn manhattan_distance(self) -> usize {
        self.row.abs() as usize + self.col.abs() as usize
    }

    /// Returns the direction that points closest to this point's coordinates.
    /// Returns None if this is the zero point or if the coordinates are on a diagional
    pub fn try_to_direction(self) -> Option<Direction> {
        if self.row > self.col.abs() {
            return Some(Direction::Down);
        }
        if self.row < -self.col.abs() {
            return Some(Direction::Up);
        }
        if self.col > self.row.abs() {
            return Some(Direction::Right);
        }
        if self.col < -self.row.abs() {
            return Some(Direction::Left);
        }
        None
    }
}

impl std::ops::Sub for SignedPoint {
    type Output = SignedPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.row - rhs.row, self.col - rhs.col)
    }
}

/// A iterator that can be converted to yield (row, col) information along with the cell's value
pub trait IntoEnumeratedCells<T>: Sized + Iterator<Item = T> + EnumeratedCellsIter {
    fn enumerate_cells(self) -> EnumeratedCells<T, Self>;
}

/// Implemented by our iterator types that also provide (row, col) information
pub trait EnumeratedCellsIter {
    /// Returns the current (row, col) location of the iterator.
    /// If calling Iterator::next, would return `None`, then this return value is allowed to be garbage.
    fn current_loc(&mut self) -> (usize, usize);
}

/// A wrapper around an iterator that can provide (row, col) information along with the value of
/// the cell
pub struct EnumeratedCells<T, I>
where
    I: Iterator<Item = T> + EnumeratedCellsIter,
{
    inner: I,
}

impl<T, I> IntoEnumeratedCells<T> for I
where
    I: Iterator<Item = T> + EnumeratedCellsIter,
{
    fn enumerate_cells(self) -> EnumeratedCells<T, Self> {
        EnumeratedCells { inner: self }
    }
}

impl<T, I> Iterator for EnumeratedCells<T, I>
where
    I: Iterator<Item = T> + EnumeratedCellsIter,
{
    type Item = (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.inner.current_loc();
        self.inner.next().map(|t| (row, col, t))
    }
}

/// An iterator that yields `n` offsets for `OffsetIter` based on a row delta, col delta and count `n`
#[derive(Clone, Debug)]
pub struct VectorOffsetIter {
    count: usize,
    end_count: usize,
    row: isize,
    col: isize,
    row_delta: isize,
    col_delta: isize,
}

impl VectorOffsetIter {
    /// Creates a VectorOffsetIter that begins one tile in `directon` off of [row, col], and
    /// continues to the edge of the `mat`
    pub fn new(count: usize, direction: Direction) -> Self {
        let offset = direction.to_unit_offsets();
        Self {
            count: 0,
            end_count: count,
            row: 0,
            col: 0,
            row_delta: offset.0,
            col_delta: offset.1,
        }
    }

    pub fn new_with_direction(count: usize, row_delta: isize, col_delta: isize) -> Self {
        Self {
            count: 0,
            end_count: count,
            row: 0,
            col: 0,
            row_delta,
            col_delta,
        }
    }
}

impl Iterator for VectorOffsetIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.end_count {
            return None;
        }
        self.row += self.row_delta;
        self.col += self.col_delta;
        self.count += 1;

        Some((self.row, self.col))
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: Into<char> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let c: char = self.get(row, col).clone().into();
                f.write_fmt(format_args!("{}", c))?;
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

impl<T> Matrix<T>
where
    usize: From<T>,
    T: Clone,
{
    pub fn pathfind(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        is_wall: impl Fn(&T) -> bool,
    ) -> Option<(Vec<(usize, usize)>, usize)> {
        let cost = |(row, col): &(usize, usize)| {
            (isize::abs(*row as isize - end.0 as isize) as usize
                + isize::abs(*col as isize - end.1 as isize) as usize)
                / 3
        };
        let successors = |(row, col): &(usize, usize)| -> _ {
            self.adjacent_neighbor_iter(*row, *col)
                .enumerate_cells()
                .filter(|(row, col, _)| !is_wall(self.get(*row, *col)))
                .map(|(row, col, _)| ((row, col), cost(&(row, col))))
        };

        pathfinding::prelude::astar(
            &start,
            |p| successors(p),
            cost,
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
        Matrix { data, cols, rows }
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

pub type IterRef<'m, T> = Iter<&'m T, std::slice::Iter<'m, T>>;
pub type IterMut<'m, T> = Iter<&'m mut T, std::slice::IterMut<'m, T>>;
pub type IterOwned<T> = Iter<T, std::vec::IntoIter<T>>;

pub struct Iter<T, I>
where
    I: Iterator<Item = T>,
{
    inner: I,
    row: usize,
    col: usize,
    col_count: usize,
}

impl<T, I> Iter<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn new(inner: I, col_count: usize) -> Self {
        Self {
            inner,
            row: 0,
            col: 0,
            col_count,
        }
    }
}

impl<T, I> Iterator for Iter<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|t| {
            // Advance row and col so `EnumeratedCellsIter::current_loc` has the right info
            self.col += 1;
            if self.col == self.col_count {
                self.col = 0;
                self.row += 1;
            }
            t
        })
    }
}

impl<T, I> EnumeratedCellsIter for Iter<T, I>
where
    I: Iterator<Item = T>,
{
    fn current_loc(&mut self) -> (usize, usize) {
        (self.row, self.col)
    }
}

pub type OffsetIterSlice<'m, 'o, T, O> =
    OffsetIter<'m, T, std::iter::Cloned<std::slice::Iter<'o, (O, O)>>, O>;

impl<'m, 'o, T, O> OffsetIterSlice<'m, 'o, T, O>
where
    O: Into<isize> + Clone,
{
    pub fn new(
        mat: &'m Matrix<T>,
        row: usize,
        col: usize,
        offsets: std::slice::Iter<'o, (O, O)>,
    ) -> Self {
        Self {
            offsets: offsets.cloned().peekable(),
            mat,
            row,
            col,
            stop_on_out_of_bounds: false,
            done: false,
        }
    }
}

/// Applies a custom set of offsets to given position, returing a refrence to a value at each
/// offset location
pub struct OffsetIter<'m, T, I, O>
where
    O: Into<isize>,
    I: Iterator<Item = (O, O)>,
{
    offsets: std::iter::Peekable<I>,
    mat: &'m Matrix<T>,
    row: usize,
    col: usize,
    /// If set to true, this iterator will end if `offsets` produces a cell that is out of range
    /// for `mat`
    stop_on_out_of_bounds: bool,
    /// Set to `true` if offsets has ran out of items, or if offsets went out of bounds and
    /// `stop_on_out_of_bounds` is set to true
    done: bool,
}

impl<T, I, O> OffsetIter<'_, T, I, O>
where
    O: Into<isize> + Clone,
    I: Iterator<Item = (O, O)>,
{
    /// Adds `off_row` an `off_col` to the center of this iterator and returns the absloute cell
    /// coordinates, if the coordinates are in bounds for `mat`
    fn apply_offset(&self, off_row: O, off_col: O) -> Option<(usize, usize)> {
        let off_row: isize = off_row.into();
        let off_col: isize = off_col.into();
        self.mat.offset(self.row, self.col, off_row, off_col)
    }

    /// Advances `self.offsets` to the next in bounds offset, so that the next call to
    /// `self.offsets.next()` is guarnteed to produce a cell in bounds.
    ///
    /// Returns `None` if there are no more offsets to apply
    fn advance_to_next_in_bounds(&mut self) -> Option<()> {
        loop {
            let (off_row, off_col) = self.offsets.peek()?;
            let (off_row, off_col) = (off_row.clone(), off_col.clone());

            let next_out_of_bounds = self.apply_offset(off_row, off_col).is_none();

            if next_out_of_bounds && self.stop_on_out_of_bounds {
                self.done = true;
                return None;
            }
            if next_out_of_bounds {
                // consume out of bounds element
                self.offsets.next();
            } else {
                // returned next element via peeking
                return self.offsets.peek().map(|_| ());
            }
        }
    }
}

impl<'m, T, I, O> EnumeratedCellsIter for OffsetIter<'m, T, I, O>
where
    O: From<i8> + Into<isize> + Clone,
    I: Iterator<Item = (O, O)>,
{
    fn current_loc(&mut self) -> (usize, usize) {
        let _ = self.advance_to_next_in_bounds();
        let (off_row, off_col) = self
            .offsets
            .peek()
            .cloned()
            .unwrap_or((0i8.into(), 0i8.into()));

        self.apply_offset(off_row.clone(), off_col.clone())
            .unwrap_or((0, 0))
    }
}

impl<'m, T, I, O> Iterator for OffsetIter<'m, T, I, O>
where
    O: Into<isize> + Clone,
    I: Iterator<Item = (O, O)>,
{
    type Item = &'m T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        self.advance_to_next_in_bounds()?;
        let (off_row, off_col) = self.offsets.next()?;

        let (row, col) = self
            .apply_offset(off_row.clone(), off_col.clone())
            .expect("advance_to_next_in_bounds left an invalid offset inside `self.offsets`");

        Some(self.mat.get(row, col))
    }
}

/// Relative offest of the 8 neightbors around a cell
const NEIGHBOR_OFFSETS: [(i8, i8); 8] = [
    // top row
    (-1, -1),
    (-1, 0),
    (-1, 1),
    // middle row
    (0, -1),
    // (0, 0), CENTER
    (0, 1),
    // bottom row
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Relative offest of the 4 directly adjacent neightbors around a cell
const ADJACENT_NEIGHBOR_OFFSETS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

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

    #[test]
    fn to_center() {
        let mat = Matrix::new_with_value(5, 5, 0u8);
        for row in 0..mat.rows() {
            let width = if row < mat.rows() / 2 {
                row
            } else {
                mat.rows() - row - 1
            };
            for col in 0..width {
                assert_eq!(mat.direction_to_center(row, col), Direction::Right);
            }
        }
        for row in 0..mat.rows() {
            let width = if row < mat.rows() / 2 {
                row
            } else {
                mat.rows() - row - 1
            };
            for col in 0..width {
                let col = mat.cols() - col - 1;
                assert_eq!(mat.direction_to_center(row, col), Direction::Left);
            }
        }

        for col in 0..mat.cols() {
            let height = if col < mat.cols() / 2 {
                col
            } else {
                mat.cols() - col - 1
            };
            for row in 0..height {
                assert_eq!(mat.direction_to_center(row, col), Direction::Down);
            }
        }
        for col in 0..mat.cols() {
            let height = if col < mat.cols() / 2 {
                col
            } else {
                mat.cols() - col - 1
            };
            for row in 0..height {
                let row = mat.rows() - row - 1;
                assert_eq!(mat.direction_to_center(row, col), Direction::Up);
            }
        }

        let max_row = mat.rows() - 1;
        let max_col = mat.cols() - 1;

        let dir = mat.direction_to_center(0, 0);
        assert!(dir == Direction::Right || dir == Direction::Down);

        let dir = mat.direction_to_center(0, max_col);
        assert!(dir == Direction::Left || dir == Direction::Down);

        let dir = mat.direction_to_center(max_row, max_col);
        assert!(dir == Direction::Left || dir == Direction::Up);

        let dir = mat.direction_to_center(max_row, 0);
        assert!(dir == Direction::Right || dir == Direction::Up);
    }

    #[test]
    fn dir_between() {
        assert_eq!(
            Direction::direction_between(0, 0, 0, 1).unwrap(),
            Direction::Right
        );
        assert_eq!(
            Direction::direction_between(0, 0, 1, 0).unwrap(),
            Direction::Down
        );
        assert_eq!(
            Direction::direction_between(4, 4, 4, 3).unwrap(),
            Direction::Left
        );
        assert_eq!(
            Direction::direction_between(4, 4, 3, 4).unwrap(),
            Direction::Up
        );
        assert!(Direction::direction_between(5, 5, 5, 5).is_none());
        assert!(Direction::direction_between(0, 0, 5, 5).is_none());
        assert!(Direction::direction_between(1, 1, 2, 2).is_none());
    }

    #[test]
    fn try_to_direction() {
        for x in -50..=50 {
            assert_eq!(SignedPoint::new(x, x).try_to_direction(), None);
        }
        assert_eq!(
            SignedPoint::new(4, 0).try_to_direction(),
            Some(Direction::Down)
        );
        assert_eq!(
            SignedPoint::new(-3, 0).try_to_direction(),
            Some(Direction::Up)
        );
        assert_eq!(
            SignedPoint::new(0, 2).try_to_direction(),
            Some(Direction::Right)
        );
        assert_eq!(
            SignedPoint::new(0, -5).try_to_direction(),
            Some(Direction::Left)
        );
    }
}
