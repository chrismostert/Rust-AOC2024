use std::{ops::Index, str::FromStr};

use itertools::Itertools;

type CoordElement = isize;
type Coord = (CoordElement, CoordElement);
#[derive(Debug)]
pub struct Grid<T> {
    inner: Vec<Vec<T>>,
    out_of_bound_value: T,
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        if x < 0
            || y < 0
            || self.inner.get(y as usize).is_none()
            || self.inner[y as usize].get(x as usize).is_none()
        {
            return &self.out_of_bound_value;
        }
        &self.inner[y as usize][x as usize]
    }
}

impl FromStr for Grid<char> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            inner: s.lines().map(|line| line.chars().collect()).collect(),
            out_of_bound_value: '\0',
        })
    }
}

impl FromStr for Grid<u32> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            inner: s
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
            out_of_bound_value: 0,
        })
    }
}

impl<T: Eq + PartialEq + Copy> Grid<T> {
    pub fn inner(&self) -> &Vec<Vec<T>> {
        &self.inner
    }

    pub fn find(&self, needle: T) -> Option<Coord> {
        for (y, yvec) in self.inner.iter().enumerate() {
            for (x, &elem) in yvec.iter().enumerate() {
                if elem == needle {
                    return Some((x as CoordElement, y as CoordElement));
                }
            }
        }
        None
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.inner.len())
            .map(|y| y as CoordElement)
            .cartesian_product((0..self.inner.len()).map(|x| x as CoordElement))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn as_coord(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}
