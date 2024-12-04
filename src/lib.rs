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
        self.0.get(&index).unwrap_or(&'.')
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
