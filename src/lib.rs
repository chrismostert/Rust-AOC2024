use std::{
    ops::{Add, Index, Mul, Rem, Sub},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use itertools::Itertools;

pub const UP: Point = Point(0, -1);
pub const UPRIGHT: Point = Point(1, -1);
pub const RIGHT: Point = Point(1, 0);
pub const DOWNRIGHT: Point = Point(1, 1);
pub const DOWN: Point = Point(0, 1);
pub const DOWNLEFT: Point = Point(-1, 1);
pub const LEFT: Point = Point(-1, 0);
pub const UPLEFT: Point = Point(-1, -1);

pub const DIAGONAL: [Point; 8] = [UP, UPRIGHT, RIGHT, DOWNRIGHT, DOWN, DOWNLEFT, LEFT, UPLEFT];
pub const ORTHOGONAL: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point(pub isize, pub isize);

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point(value.0, value.1)
    }
}

impl<T: Into<Point>> Add<T> for Point {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let Point(x, y): Point = rhs.into();
        Point(self.0 + x, self.1 + y)
    }
}

impl<T: Into<Point>> Sub<T> for Point {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let Point(x, y): Point = rhs.into();
        Point(self.0 - x, self.1 - y)
    }
}

impl<T: Into<i64>> Mul<T> for Point {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mul: i64 = rhs.into();
        Point(self.0 * mul as isize, self.1 * mul as isize)
    }
}

impl<T: Into<Point>> Rem<T> for Point {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        let Point(x, y): Point = rhs.into();
        let x_w = self.0 % x;
        let y_w = self.1 % y;
        Point(
            if x_w >= 0 { x_w } else { x + x_w },
            if y_w >= 0 { y_w } else { y + y_w },
        )
    }
}

impl Point {
    pub fn clockwise(self) -> Self {
        Point(-self.1, self.0)
    }

    pub fn orthogonal(self) -> impl Iterator<Item = Self> {
        ORTHOGONAL.into_iter().map(move |dir| dir + self)
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
    out_of_bound_value: T,
}

impl<T, P: Into<Point>> Index<P> for Grid<T> {
    type Output = T;

    fn index(&self, point: P) -> &Self::Output {
        let Point(x, y) = point.into();
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return &self.out_of_bound_value;
        }
        &self.inner[x as usize + y as usize * self.width]
    }
}

impl<T: FromStr + Default> FromStr for Grid<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            inner: s
                .lines()
                .flat_map(|line| {
                    (0..line.len()).map(|idx| {
                        line[idx..=idx].parse().map_err(|_| {
                            anyhow!(
                                "Failed to parse '{}' into the desired type.",
                                &line[idx..=idx]
                            )
                        })
                    })
                })
                .try_collect()?,
            width: s.lines().next().context("No lines")?.len(),
            height: s.lines().count(),
            out_of_bound_value: T::default(),
        })
    }
}

impl<T: Eq + PartialEq + Copy> Grid<T> {
    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    pub fn find(&self, needle: T) -> Option<Point> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .find(|&(x, y)| self[(x as isize, y as isize)] == needle)
            .map(|(x, y)| Point(x as isize, y as isize))
    }

    pub fn coords(&self) -> impl Iterator<Item = Point> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| Point(x as isize, y as isize))
    }

    pub fn items(&self) -> impl Iterator<Item = (&T, Point)> {
        self.iter().zip(self.coords())
    }
}
