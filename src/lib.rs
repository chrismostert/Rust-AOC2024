use std::{ops::Index, str::FromStr};

use anyhow::{anyhow, Context};
use itertools::Itertools;

type CoordElement = isize;
type Coord = (CoordElement, CoordElement);
#[derive(Debug)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
    out_of_bound_value: T,
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): Coord) -> &Self::Output {
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

    pub fn find(&self, needle: T) -> Option<Coord> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .find(|(x, y)| self.inner[x + y * self.width] == needle)
            .map(|(x, y)| (x as CoordElement, y as CoordElement))
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| (x as CoordElement, y as CoordElement))
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
