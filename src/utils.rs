#![allow(dead_code)]

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2D {
    pub x: isize,
    pub y: isize
}

impl Point2D {

    pub const fn new(x: isize, y: isize) -> Self {
        Point2D {
            x,
            y
        }
    }

    pub fn abs(&self) -> Point2D {
        Self {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

    pub fn adjacent_points(&self) -> [Point2D; 4] {
        [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1)
        ].map(|(x_d, y_d)| {
            Point2D::new(self.x + x_d, self.y + y_d)
        })
    }

    pub fn x_index(&self) -> usize {
        self.x as usize
    }

    pub fn y_index(&self) -> usize {
        self.y as usize
    }

    pub fn column(&self) -> isize {
        self.x
    }

    pub fn column_index(&self) -> usize {
        self.x as usize
    }

    pub fn negate(&self) -> Point2D {
        Self {
            x: -self.x,
            y: -self.y
        }
    }

    pub fn normalize(&self) -> Point2D {
        let gcd = gcd(self.x.abs() as usize, self.y.abs() as usize) as isize;
        Self {
            x: self.x / gcd,
            y: self.y / gcd
        }
    }

    pub fn row(&self) -> isize {
        self.y
    }

    pub fn row_index(&self) -> usize {
        self.y as usize
    }

    pub fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

}

impl Add<Point2D> for Point2D {

    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

}

impl Add<&Point2D> for Point2D {

    type Output = Point2D;

    fn add(self, rhs: &Point2D) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

}

impl AddAssign<Point2D> for Point2D {

    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

}

impl AddAssign<&Point2D> for Point2D {

    fn add_assign(&mut self, rhs: &Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

}

impl Mul<isize> for Point2D {

    type Output = Point2D;

    fn mul(self, rhs: isize) -> Self::Output {
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }

}

impl Mul<&isize> for Point2D {

    type Output = Point2D;

    fn mul(self, rhs: &isize) -> Self::Output {
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }

}

impl MulAssign<isize> for Point2D {

    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }

}

impl MulAssign<&isize> for Point2D {

    fn mul_assign(&mut self, rhs: &isize) {
        self.x *= rhs;
        self.y *= rhs;
    }

}

impl Sub<Point2D> for Point2D {

    type Output = Point2D;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }

}

impl Sub<&Point2D> for Point2D {

    type Output = Point2D;

    fn sub(self, rhs: &Point2D) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }

}

impl SubAssign<Point2D> for Point2D {

    fn sub_assign(&mut self, rhs: Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
    
}

impl SubAssign<&Point2D> for Point2D {

    fn sub_assign(&mut self, rhs: &Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
    
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize
}

impl Point3D {

    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Point3D {
            x,
            y,
            z
        }
    }

    pub fn abs(&self) -> Point3D {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs()
        }
    }

    pub fn adjacent_points(&self) -> [Point3D; 6] {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1)
        ].map(|(x_d, y_d, z_d)| {
            Point3D::new(self.x + x_d, self.y + y_d, self.z + z_d)
        })
    }

    pub fn x_index(&self) -> usize {
        self.x as usize
    }

    pub fn y_index(&self) -> usize {
        self.y as usize
    }

    pub fn z_index(&self) -> usize {
        self.z as usize
    }

    pub fn negate(&self) -> Point3D {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    pub fn normalize(&self) -> Point3D {
        let gcd = gcd(gcd(self.x.abs() as usize, self.y.abs() as usize), self.z.abs() as usize) as isize;
        Self {
            x: self.x / gcd,
            y: self.y / gcd,
            z: self.z / gcd
        }
    }

    pub fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

}

impl Add<Point3D> for Point3D {

    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }

}

impl Add<&Point3D> for Point3D {

    type Output = Point3D;

    fn add(self, rhs: &Point3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }

}

impl AddAssign<Point3D> for Point3D {

    fn add_assign(&mut self, rhs: Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }

}

impl AddAssign<&Point3D> for Point3D {

    fn add_assign(&mut self, rhs: &Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }

}

impl Sub<Point3D> for Point3D {

    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }

}

impl Sub<&Point3D> for Point3D {

    type Output = Point3D;

    fn sub(self, rhs: &Point3D) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }

}

impl SubAssign<Point3D> for Point3D {

    fn sub_assign(&mut self, rhs: Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
    
}

impl SubAssign<&Point3D> for Point3D {

    fn sub_assign(&mut self, rhs: &Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
    
}

pub trait Grid<T: ?Sized>: Index<T> {

    fn col_count(&self) -> usize;

    fn row_count(&self) -> usize;

    fn contains(&self, point: T) -> bool;

}

pub struct Grid2DBorrowed<'a> {
    input_bytes: &'a [u8],
    row_count: usize,
    col_count: usize,
    row_stride: usize
}

impl<'a> Grid2DBorrowed<'a> {

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

        Self {
            input_bytes: bytes,
            row_count: bytes.len().div_ceil(row_length + line_escape_length),
            col_count: row_length,
            row_stride: row_length + line_escape_length
        }
    }

    pub fn to_owned(&self) -> Grid2D<u8> {
        Grid2D {
            storage: self.input_bytes.to_owned(),
            row_count: self.row_count,
            col_count: self.col_count,
            row_stride: self.row_stride
        }
    }

}

impl<'a> Grid<Point2D> for Grid2DBorrowed<'a> {

    fn col_count(&self) -> usize {
        self.col_count
    }

    fn row_count(&self) -> usize {
        self.row_count
    }
    
    fn contains(&self, point: Point2D) -> bool {
        return point.row_index() < self.row_count &&
               point.column_index() < self.col_count
    }

}

impl<'a> Index<Point2D> for Grid2DBorrowed<'a> {

    type Output = u8;

    fn index(&self, index: Point2D) -> &Self::Output {
        &self.input_bytes[index.row_index() * self.row_stride + index.column_index()]
    }

}

pub struct Grid2D<T> {
    storage: Vec<T>,
    row_count: usize,
    col_count: usize,
    row_stride: usize
}

impl<T: Copy + Default> Grid2D<T> {

    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            storage: vec![T::default(); row_count * col_count],
            row_count,
            col_count,
            row_stride: col_count
        }
    }

}

impl<T> Grid2D<T> {

    pub fn backing_store(&self) -> &[T] {
        &self.storage
    }

    pub fn backing_store_mut(&mut self) -> &mut [T] {
        &mut self.storage
    }

}

impl<T> Grid<Point2D> for Grid2D<T> {

    fn col_count(&self) -> usize {
        self.col_count
    }

    fn row_count(&self) -> usize {
        self.row_count
    }
    
    fn contains(&self, point: Point2D) -> bool {
        return point.row_index() < self.row_count &&
               point.column_index() < self.col_count
    }

}

impl<T> Index<Point2D> for Grid2D<T> {

    type Output = T;

    fn index(&self, index: Point2D) -> &Self::Output {
        &(*self.storage)[index.row_index() * self.row_stride + index.column_index()]
    }

}

impl<T> IndexMut<Point2D> for Grid2D<T> {

    fn index_mut(&mut self, index: Point2D) -> &mut Self::Output {
        &mut (*self.storage)[index.row_index() * self.row_stride + index.column_index()]
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