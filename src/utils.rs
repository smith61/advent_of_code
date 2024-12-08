#![allow(dead_code)]

use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Sub, SubAssign};

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

enum Grid2DStorage<'a, T> {
    Owned(Vec<T>),
    Borrowed(&'a [T])
}

impl<'a, T> Deref for Grid2DStorage<'a, T> {

    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Grid2DStorage::Owned(v) => &v[..],
            Grid2DStorage::Borrowed(v) => v
        }
    }

}

impl<'a, T> DerefMut for Grid2DStorage<'a, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Grid2DStorage::Owned(v) => &mut v[..],
            Grid2DStorage::Borrowed(_) => panic!("Can't modify borrowed storage")
        }
    }

}

pub struct Grid2DBorrowed<'a> {
    input_bytes: &'a [u8],
    row_count: usize,
    col_count: usize,
    row_stride: usize
}

impl<'a> Grid2DBorrowed<'a> {

    pub fn from_input_lines(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let new_line_index =
            bytes.iter()
                 .enumerate()
                 .find(|v| *v.1 == b'\r')
                 .unwrap().0;

        let row_count = bytes.len() / (new_line_index + 1);
        Self {
            input_bytes: bytes,
            row_count,
            col_count: new_line_index,
            row_stride: new_line_index + 2
        }
    }

}

impl<'a> Grid2DBorrowed<'a> {

    pub fn col_count(&self) -> usize {
        self.col_count
    }

    pub fn row_count(&self) -> usize {
        self.row_count
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
    col_count: usize
}

impl<T: Copy + Default> Grid2D<T> {

    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            storage: vec![T::default(); row_count * col_count],
            row_count,
            col_count
        }
    }

}

impl<T> Grid2D<T> {

    pub fn col_count(&self) -> usize {
        self.col_count
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

}

impl<T> Index<Point2D> for Grid2D<T> {

    type Output = T;

    fn index(&self, index: Point2D) -> &Self::Output {
        &(*self.storage)[index.row_index() * self.col_count + index.column_index()]
    }

}

impl<T> IndexMut<Point2D> for Grid2D<T> {

    fn index_mut(&mut self, index: Point2D) -> &mut Self::Output {
        &mut (*self.storage)[index.row_index() * self.col_count + index.column_index()]
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