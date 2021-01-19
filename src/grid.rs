use std::fmt::{Display, Formatter};
use std::process::exit;
use macroquad::prelude::rand::*;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Alive,
    Dead(bool),
}

pub enum Error {
    OutOfBounds
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::OutOfBounds => {
                f.write_str("Out of Bounds")?;
            }
        }

        Ok(())
    }
}

pub struct Grid {
    pub cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::Dead(false); width * height];
        Self {
            cells,
            width,
            height
        }
    }

    pub fn awaken(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if x >= self.width || y >= self.height {
            println!("Out of bounds");
            return Err(Error::OutOfBounds);
        }

        let index = self.index_from_pos(x, y);

        self.cells[index] = Cell::Alive;

        Ok(())
    }

    pub fn dying(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if x >= self.width || y >= self.height {
            println!("Out of bounds");
            return Err(Error::OutOfBounds);
        }

        let index = self.index_from_pos(x, y);

        self.cells[index] = Cell::Dead(true);

        Ok(())
    }

    pub fn randomize(&mut self, size: usize) {
        for i in 0..size {
            self.awaken(gen_range::<usize>(0, self.width - 1), gen_range::<usize>(0, self.height - 1));
        }
    }

    pub fn step(&mut self) {
        let mut new_grid = Self::new(self.width, self.height);
        for (i, cell) in self.cells.clone().iter().enumerate() {
            let (x, y) = self.pos_from_index(i);

            let left = match x {
                0 => self.width - 1,
                _ => x - 1
            };
            let right = match x {
                _ if self.width - 1 == x => 0,
                _ => x + 1
            };
            let top = match y {
                0 => self.height - 1,
                _ => y - 1,
            };
            let bottom = match y {
                _ if self.height - 1 == y  => 0,
                _ => y + 1,
            };

            let neighbours = vec![
                self.cells[self.index_from_pos(left, top)].clone(),
                self.cells[self.index_from_pos(x, top)].clone(),
                self.cells[self.index_from_pos(right, top)].clone(),
                self.cells[self.index_from_pos(left, bottom)].clone(),
                self.cells[self.index_from_pos(x, bottom)].clone(),
                self.cells[self.index_from_pos(right, bottom)].clone(),
                self.cells[self.index_from_pos(right, y)].clone(),
                self.cells[self.index_from_pos(left, y)].clone(),
            ];

            let alive = neighbours.iter().fold(0, |i, cell| if *cell == Cell::Alive { i + 1 } else { i });

            match cell {
                Cell::Alive => {
                    match alive {
                        2 => new_grid.awaken(x, y),
                        3 => new_grid.awaken(x, y),
                        _ => new_grid.dying(x, y),
                    }
                },
                Cell::Dead(_) => {
                    match alive {
                        3 => new_grid.awaken(x, y),
                        _ => Ok(()),
                    }
                }
            };
        }


        self.cells = new_grid.cells.clone();
    }


    fn index_from_pos(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn pos_from_index(&self, index: usize) -> (usize, usize) {
        (
            index % self.width,
            index / self.width
        )
    }
}

