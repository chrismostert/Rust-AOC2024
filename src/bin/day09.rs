use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, Clone, Copy)]
struct File(Option<u64>, u64);

#[derive(Debug)]
struct Disk(Vec<File>);
impl FromStr for Disk {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut res, mut idx, mut file) = (Vec::new(), 0, true);
        for char in s.chars().filter(|c| c.is_ascii_digit()) {
            let size = char.to_digit(10).context("Not a digit")? as u64;
            if file {
                res.push(File(Some(idx), size));
                idx += 1;
            } else {
                res.push(File(None, size));
            }
            file = !file;
        }
        Ok(Disk(res))
    }
}

impl Disk {
    fn as_blocks(&self) -> Self {
        Disk(
            self.0
                .iter()
                .flat_map(|&File(file_no, file_size)| (0..file_size).map(move |_| File(file_no, 1)))
                .collect(),
        )
    }

    fn defrag(&mut self) -> &Self {
        for r_idx in (0..self.0.len()).rev() {
            for l_idx in 0..r_idx {
                let file_r @ File(no_r, size_r) = self.0[r_idx];
                let File(no_l, size_l) = self.0[l_idx];

                if no_r.is_some() && no_l.is_none() && size_r <= size_l {
                    self.0[l_idx] = file_r;
                    self.0[r_idx] = File(None, size_r);
                    let remaining_space = size_l - size_r;
                    if remaining_space > 0 {
                        self.0.insert(l_idx + 1, File(None, remaining_space));
                    }
                    break;
                }
            }
        }
        self
    }

    fn hash(&self) -> u64 {
        let (mut idx, mut res) = (0, 0);
        for file in &self.0 {
            match file {
                File(None, size) => idx += size,
                File(Some(file_no), size) => {
                    for _ in 0..*size {
                        res += idx * file_no;
                        idx += 1;
                    }
                }
            }
        }
        res
    }
}

fn main() {
    let mut input = include_str!("../../inputs/day09.txt")
        .parse::<Disk>()
        .unwrap();

    println!("Part 1: {}", input.as_blocks().defrag().hash());
    println!("Part 2: {}", input.defrag().hash());
}
