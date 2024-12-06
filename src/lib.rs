use std::{
    collections::HashMap,
    ops::{Deref, Index},
    str::FromStr,
};

type CoordElement = isize;
type Coord = (CoordElement, CoordElement);
pub struct CharGrid(HashMap<Coord, char>);

impl Index<Coord> for CharGrid {
    type Output = char;

    fn index(&self, index: Coord) -> &Self::Output {
        self.0.get(&index).unwrap_or(&'\x00')
    }
}

impl FromStr for CharGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CharGrid(
            s.lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as CoordElement, y as CoordElement), c))
                })
                .collect(),
        ))
    }
}

impl Deref for CharGrid {
    type Target = HashMap<Coord, char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CharGrid {
    pub fn find_char(&self, character: char) -> Option<Coord> {
        self.iter()
            .find(|(_, &c)| c == character)
            .map(|found| *found.0)
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
            Direction::Left => (-1, 0)
        }
    }
}
