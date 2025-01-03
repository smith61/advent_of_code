#![allow(dead_code)]

use std::{collections::BinaryHeap, marker::PhantomData, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}};

use itertools::Itertools;


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector<const DIMENSIONS: usize> {
    pub values: [isize; DIMENSIONS]
}

impl<const DIMENSIONS: usize> Default for Vector<DIMENSIONS> {

    fn default() -> Self {
        Self {
            values: [0; DIMENSIONS]
        }
    }

}

impl Vector<2> {

    pub const fn new(x: isize, y: isize) -> Self {
        Self {
            values: [x, y]
        }
    }

    pub const fn x(&self) -> isize {
        self.values[0]
    }

    pub const fn x_index(&self) -> usize {
        self.values[0] as usize
    }

    pub const fn y(&self) -> isize {
        self.values[1]
    }

    pub const fn y_index(&self) -> usize {
        self.values[1] as usize
    }

    pub const fn column(&self) -> isize {
        self.values[0]
    }

    pub const fn column_index(&self) -> usize {
        self.values[0] as usize
    }

    pub const fn row(&self) -> isize {
        self.values[1]
    }

    pub const fn row_index(&self) -> usize {
        self.values[1] as usize
    }

    pub fn adjacent_points(&self) -> [Self; 4] {
        [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1)
        ].map(|(x_d, y_d)| {
            Self::new(self.x() + x_d, self.y() + y_d)
        })
    }

}

impl From<(isize, isize)> for Vector<2> {

    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }

}

impl From<(usize, usize)> for Vector<2> {

    fn from(value: (usize, usize)) -> Self {
        assert!(value.0 <= isize::MAX as usize);
        assert!(value.1 <= isize::MAX as usize);

        Self::new(value.0 as isize, value.1 as isize)
    }

}

impl Vector<3> {

    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self {
            values: [x, y, z]
        }
    }

    pub const fn x(&self) -> isize {
        self.values[0]
    }

    pub const fn x_index(&self) -> usize {
        self.values[0] as usize
    }

    pub const fn y(&self) -> isize {
        self.values[1]
    }

    pub const fn y_index(&self) -> usize {
        self.values[1] as usize
    }

    pub const fn z(&self) -> isize {
        self.values[2]
    }

    pub const fn z_index(&self) -> usize {
        self.values[2] as usize
    }

    pub fn adjacent_points(&self) -> [Self; 6] {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1)
        ].map(|(x_d, y_d, z_d)| {
            Self::new(self.x() + x_d, self.y() + y_d, self.z() + z_d)
        })
    }

}

impl From<(isize, isize, isize)> for Vector<3> {

    fn from(value: (isize, isize, isize)) -> Self {
        Self::new(value.0, value.1, value.2)
    }

}

impl From<(usize, usize, usize)> for Vector<3> {

    fn from(value: (usize, usize, usize)) -> Self {
        assert!(value.0 <= isize::MAX as usize);
        assert!(value.1 <= isize::MAX as usize);
        assert!(value.2 <= isize::MAX as usize);

        Self::new(value.0 as isize, value.1 as isize, value.2 as isize)
    }

}

impl<const DIMENSIONS: usize> Vector<DIMENSIONS> {

    pub fn abs(&self) -> Self {
        Self {
            values: self.values.map(|v| v.abs())
        }
    }

    pub fn normalize(&self) -> Self {
        let gcd =
            self.values
                .iter()
                .copied()
                .reduce(|l, r| gcd(l.abs() as usize, r.abs() as usize) as isize)
                .unwrap();

        Self {
            values: self.values.map(|v| v / gcd)
        }
    }

    pub fn manhattan_distance(&self) -> isize {
        self.values.iter().map(|v| v.abs()).sum::<isize>()
    }

}

impl<const DIMENSIONS: usize> Add<Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    type Output = Self;

    fn add(self, rhs: Vector<DIMENSIONS>) -> Self::Output {
        let mut values = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            values[i] = self.values[i] + rhs.values[i];
        }

        Self {
            values
        }
    }

}

impl<const DIMENSIONS: usize> Add<&Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    type Output = Self;

    fn add(self, rhs: &Vector<DIMENSIONS>) -> Self::Output {
        let mut values = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            values[i] = self.values[i] + rhs.values[i];
        }

        Self {
            values
        }
    }

}

impl<const DIMENSIONS: usize> AddAssign<Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    fn add_assign(&mut self, rhs: Vector<DIMENSIONS>) {
        for i in 0..DIMENSIONS {
            self.values[i] += rhs.values[i];
        }
    }

}

impl<const DIMENSIONS: usize> AddAssign<&Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    fn add_assign(&mut self, rhs: &Vector<DIMENSIONS>) {
        for i in 0..DIMENSIONS {
            self.values[i] += rhs.values[i];
        }
    }

}

impl<const DIMENSIONS: usize> Mul<isize> for Vector<DIMENSIONS> {

    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        let mut values = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            values[i] = self.values[i] * rhs;
        }

        Self {
            values
        }
    }

}

impl<const DIMENSIONS: usize> Mul<&isize> for Vector<DIMENSIONS> {

    type Output = Self;

    fn mul(self, rhs: &isize) -> Self::Output {
        let mut values = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            values[i] = self.values[i] * rhs;
        }

        Self {
            values
        }
    }

}

impl<const DIMENSIONS: usize> MulAssign<isize> for Vector<DIMENSIONS> {

    fn mul_assign(&mut self, rhs: isize) {
        for i in 0..DIMENSIONS {
            self.values[i] *= rhs;
        }
    }

}

impl<const DIMENSIONS: usize> MulAssign<&isize> for Vector<DIMENSIONS> {

    fn mul_assign(&mut self, rhs: &isize) {
        for i in 0..DIMENSIONS {
            self.values[i] *= rhs;
        }
    }

}

impl<const DIMENSIONS: usize> Sub<Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    type Output = Self;

    fn sub(self, rhs: Vector<DIMENSIONS>) -> Self::Output {
        let mut values = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            values[i] = self.values[i] - rhs.values[i];
        }

        Self {
            values
        }
    }

}

impl<const DIMENSIONS: usize> Sub<&Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    type Output = Self;

    fn sub(self, rhs: &Vector<DIMENSIONS>) -> Self::Output {
        let mut values = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            values[i] = self.values[i] - rhs.values[i];
        }

        Self {
            values
        }
    }

}

impl<const DIMENSIONS: usize> SubAssign<Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    fn sub_assign(&mut self, rhs: Vector<DIMENSIONS>) {
        for i in 0..DIMENSIONS {
            self.values[i] -= rhs.values[i];
        }
    }

}

impl<const DIMENSIONS: usize> SubAssign<&Vector<DIMENSIONS>> for Vector<DIMENSIONS> {

    fn sub_assign(&mut self, rhs: &Vector<DIMENSIONS>) {
        for i in 0..DIMENSIONS {
            self.values[i] -= rhs.values[i];
        }
    }

}

impl<const DIMENSIONS: usize> std::fmt::Display for Vector<DIMENSIONS> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for i in 0..DIMENSIONS {
            if i != 0 {
                write!(f, ",")?;
            }

            write!(f, "{}", self.values[i])?;
        }

        write!(f, ")")
    }

}

pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;

pub struct Matrix<const DIMENSIONS: usize, S: ?Sized, T> {
    pub grid_bounds: [usize; DIMENSIONS],
    pub grid_strides: [usize; DIMENSIONS],
    _bs: PhantomData<*const T>,
    storage: S
}

impl<T: Clone + Default> Matrix<2, Vec<T>, T> {

    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            grid_bounds: [col_count, row_count],
            grid_strides: [1, col_count],
            _bs: PhantomData::default(),
            storage: vec![T::default(); row_count * col_count]
        }
    }

}

impl<S, T> Matrix<2, S, T> {

    pub fn row_count(&self) -> usize {
        self.grid_bounds[1]
    }

    pub fn col_count(&self) -> usize {
        self.grid_bounds[0]
    }

    pub fn cell_iter(&self) -> impl Iterator<Item = Vector2> {
        (0..self.row_count())
            .cartesian_product(0..self.col_count())
            .map(|(r, c)| (c, r).into())
    }
}

impl<T: Clone + Default> Matrix<3, Vec<T>, T> {

    pub fn new(x_count: usize, y_count: usize, z_count: usize) -> Self {
        Self {
            grid_bounds: [x_count, y_count, z_count],
            grid_strides: [1, x_count, y_count],
            _bs: PhantomData::default(),
            storage: vec![T::default(); x_count * y_count * z_count]
        }
    }

}

impl<'a> Matrix<2, &'a [u8], u8> {

    pub fn from_input_lines(input: &'a str) -> Self {
        let bytes = input.trim().as_bytes();
        let (row_length, line_escape_length) = {
            let mut index = 0;
            let line_escape_length;
            loop {
                if bytes[index] == b'\r' {
                    line_escape_length = 2;
                    break;

                } else if bytes[index] == b'\n' {
                    line_escape_length = 1;
                    break;
                }

                index += 1;
            }

            (index, line_escape_length)
        };

        assert!((bytes.len() % (row_length + line_escape_length)) == row_length);

        let row_count = bytes.len().div_ceil(row_length + line_escape_length);

        Self::new(bytes, [row_length, row_count], [1, row_length + line_escape_length])
    }

}

impl<S, T> Matrix<3, S, T> {

    pub fn x_count(&self) -> usize {
        self.grid_bounds[0]
    }

    pub fn y_count(&self) -> usize {
        self.grid_bounds[1]
    }

    pub fn z_count(&self) -> usize {
        self.grid_bounds[2]
    }

    pub fn cell_iter(&self) -> impl Iterator<Item = Vector3> {
        (0..self.z_count())
            .cartesian_product(0..self.y_count())
            .cartesian_product(0..self.x_count())
            .map(|((z, y), x)| (x, y, z).into())
    }

}

impl<'a, const DIMENSIONS: usize, T> Matrix<DIMENSIONS, &'a [T], T> {

    pub fn new(storage: &'a [T], grid_bounds: [usize; DIMENSIONS], grid_strides: [usize; DIMENSIONS]) -> Self {
        Self {
            grid_bounds,
            grid_strides,
            _bs: PhantomData::default(),
            storage
        }
    }

}

impl<'a, const DIMENSIONS: usize, T: Clone> Matrix<DIMENSIONS, &'a [T], T> {

    pub fn to_owned(self) -> Matrix<DIMENSIONS, Vec<T>, T> {
        Matrix {
            grid_bounds: self.grid_bounds,
            grid_strides: self.grid_strides,
            _bs: PhantomData::default(),
            storage: self.storage.to_vec()
        }
    }

}

impl<const DIMENSIONS: usize, S: ?Sized, T> Matrix<DIMENSIONS, S, T> {

    pub fn contains(&self, vector: Vector<DIMENSIONS>) -> bool {
        for dimension in 0..DIMENSIONS {
            if vector.values[dimension] < 0 ||
               (vector.values[dimension] as usize) >= self.grid_bounds[dimension] {

                return false;
            }
        }

        true
    }

}

impl<const DIMENSIONS: usize, S: AsRef<[T]>, T> Matrix<DIMENSIONS, S, T> {

    pub fn backing_store(&self) -> &[T] {
        self.storage.as_ref()
    }

}

impl<const DIMENSIONS: usize, S: AsRef<[T]> + AsMut<[T]>, T> Matrix<DIMENSIONS, S, T> {

    pub fn backing_store_mut(&mut self) -> &mut [T] {
        self.storage.as_mut()
    }

}

impl<const DIMENSIONS: usize, S: AsRef<[T]> + ?Sized, T, V: Into<Vector<DIMENSIONS>>> Index<V> for Matrix<DIMENSIONS, S, T> {

    type Output = T;

    fn index(&self, index: V) -> &Self::Output {
        let index = index.into();

        assert!(self.contains(index));

        let storage = self.storage.as_ref();
        let mut offset = 0;
        for dimension in (0..DIMENSIONS).rev() {
            offset += index.values[dimension] as usize;
            offset *= self.grid_strides[dimension];
        }

        &storage[offset]
    }

}

impl<const DIMENSIONS: usize, S: AsRef<[T]> + AsMut<[T]> + ?Sized, T, V: Into<Vector<DIMENSIONS>>> IndexMut<V> for Matrix<DIMENSIONS, S, T> {
    
    fn index_mut(&mut self, index: V) -> &mut Self::Output {
        let index = index.into();

        assert!(self.contains(index));

        let storage = self.storage.as_mut();
        let mut offset = 0;
        for dimension in (0..DIMENSIONS).rev() {
            offset += index.values[dimension] as usize;
            offset *= self.grid_strides[dimension];
        }

        &mut storage[offset]
    }

}

pub type Matrix2DBorrowed<'a, T> = Matrix<2, &'a [T], T>;
pub type Matrix2DOwned<T> = Matrix<2, Vec<T>, T>;
pub type Matrix3DBorrowed<'a, T> = Matrix<3, &'a [T], T>;
pub type Matrix3DOwned<T> = Matrix<3, Vec<T>, T>;

#[derive(Clone, Debug)]
struct DijkstraQueueNode<C: Ord, T>(C, T);

impl<C: Ord, T> PartialEq for DijkstraQueueNode<C, T> {

    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

}

impl<C: Ord, T> PartialOrd for DijkstraQueueNode<C, T> {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }

}

impl<C: Ord, T> Eq for DijkstraQueueNode<C, T> {

}

impl<C: Ord, T> Ord for DijkstraQueueNode<C, T> {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }

}

#[derive(Clone, Debug, Default)]
pub struct DijkstraQueue<C: Ord, T>(BinaryHeap<DijkstraQueueNode<C, T>>);

impl<C: Ord, T> DijkstraQueue<C, T> {

    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn pop(&mut self) -> Option<(C, T)> {
        self.0.pop().map(|DijkstraQueueNode(cost, element)| (cost, element))
    }

    pub fn push(&mut self, cost: C, element: T) {
        self.0.push(DijkstraQueueNode(cost, element));
    }

}

pub fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        v

    } else if v == 0 {
        u

    } else {
        let i = u.trailing_zeros();  u >>= i;
        let j = v.trailing_zeros();  v >>= j;
        let k = std::cmp::min(i, j);
    
        loop {
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }

            v -= u;
            if v == 0 {
                return u << k;
            }
    
            v >>= v.trailing_zeros();
        }
    }
}

pub mod z3 {

    use z3::Context;
    use z3::ast::Int;

    pub fn z3_abs<'a>(context: &'a Context, val: Int<'a>) -> Int<'a> {
        let zero = Int::from_i64(context, 0);
        val.lt(&zero).ite(&val.unary_minus(), &val)
    }

}